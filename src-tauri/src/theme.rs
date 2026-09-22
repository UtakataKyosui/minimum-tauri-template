use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_store::StoreExt;
use tauri_specta::Event;

const STORE_FILE: &str = "settings.json";
const STORE_KEY: &str = "theme";

/// `tauri::Theme` に System が無いため、OS 追従を表現するための独自設定値。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum ThemeSetting {
    System,
    Light,
    Dark,
}

impl Default for ThemeSetting {
    fn default() -> Self {
        Self::System
    }
}

impl ThemeSetting {
    /// System は「ウィンドウのテーマを固定しない」を意味するため None を返す。
    pub fn to_window_theme(self) -> Option<tauri::Theme> {
        match self {
            ThemeSetting::System => None,
            ThemeSetting::Light => Some(tauri::Theme::Light),
            ThemeSetting::Dark => Some(tauri::Theme::Dark),
        }
    }
}

/// `tauri::Theme` は specta::Type を実装せず non_exhaustive でもあるため、
/// フロントへ渡す解決済みテーマはこの型に写して扱う。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum ResolvedTheme {
    Light,
    Dark,
}

impl From<tauri::Theme> for ResolvedTheme {
    fn from(theme: tauri::Theme) -> Self {
        match theme {
            tauri::Theme::Dark => ResolvedTheme::Dark,
            _ => ResolvedTheme::Light,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ThemeStatus {
    pub setting: ThemeSetting,
    pub resolved: ResolvedTheme,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, specta::Type, Event)]
pub struct ThemeChanged(pub ResolvedTheme);

pub struct ThemeState(pub Mutex<ThemeSetting>);

impl ThemeState {
    pub fn new(setting: ThemeSetting) -> Self {
        Self(Mutex::new(setting))
    }

    pub fn get(&self) -> ThemeSetting {
        *self.0.lock().unwrap()
    }

    pub fn set(&self, setting: ThemeSetting) {
        *self.0.lock().unwrap() = setting;
    }
}

/// 初回描画前に <html> へクラスを付けるスクリプト。React の初期化を待つと
/// 一瞬だけ既定のライトテーマが見えるため、ウィンドウ生成時に注入する。
/// System のときだけ OS 値の問い合わせが要る。明示指定は設定値がそのまま答え。
pub fn initialization_script(setting: ThemeSetting) -> String {
    let setting = match setting {
        ThemeSetting::System => "system",
        ThemeSetting::Light => "light",
        ThemeSetting::Dark => "dark",
    };
    format!(
        r#"(() => {{
  const setting = "{setting}";
  const resolved = setting === "system"
    ? (window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light")
    : setting;
  document.documentElement.classList.add(resolved);
}})();"#
    )
}

/// 設定をウィンドウへ反映し、実際に適用されたテーマを返す。
pub fn apply_theme(window: &tauri::WebviewWindow, setting: ThemeSetting) -> ResolvedTheme {
    let _ = window.set_theme(setting.to_window_theme());
    match setting {
        // 明示指定は set_theme が確定させるため、ウィンドウへの問い合わせを待たずに導出する。
        ThemeSetting::Light => ResolvedTheme::Light,
        ThemeSetting::Dark => ResolvedTheme::Dark,
        ThemeSetting::System => resolved_theme(window),
    }
}

fn resolved_theme(window: &tauri::WebviewWindow) -> ResolvedTheme {
    window.theme().unwrap_or(tauri::Theme::Light).into()
}

#[tauri::command]
#[specta::specta]
pub fn set_theme(
    setting: ThemeSetting,
    window: tauri::WebviewWindow,
    state: tauri::State<ThemeState>,
) -> ResolvedTheme {
    state.set(setting);
    let resolved = apply_theme(&window, setting);
    save_theme(window.app_handle(), setting);
    resolved
}

/// 設定値と、それを解決した実際のテーマの両方を返す。
/// System のとき、フロントは起動直後の解決値をこれで知る。
#[tauri::command]
#[specta::specta]
pub fn get_theme(window: tauri::WebviewWindow, state: tauri::State<ThemeState>) -> ThemeStatus {
    ThemeStatus {
        setting: state.get(),
        resolved: resolved_theme(&window),
    }
}

pub fn save_theme(app: &tauri::AppHandle, setting: ThemeSetting) {
    let Ok(store) = app.store(STORE_FILE) else {
        return;
    };
    if let Ok(value) = serde_json::to_value(setting) {
        store.set(STORE_KEY, value);
        let _ = store.save();
    }
}

pub fn load_theme(app: &tauri::AppHandle) -> ThemeSetting {
    let Ok(store) = app.store(STORE_FILE) else {
        return ThemeSetting::default();
    };
    store
        .get(STORE_KEY)
        .and_then(|value| serde_json::from_value::<ThemeSetting>(value).ok())
        .unwrap_or_default()
}

/// OS のテーマ変更を監視する。System のときだけフロントへ通知する。
/// 明示指定中の反映は set_theme の戻り値が担うため、ここからは通知しない。
pub fn handle_theme_changed(window: &tauri::WebviewWindow) {
    let emitter = window.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::ThemeChanged(theme) = event {
            if let Some(state) = emitter.try_state::<ThemeState>() {
                if state.get() == ThemeSetting::System {
                    let _ = ThemeChanged(ResolvedTheme::from(*theme)).emit(&emitter);
                }
            }
        }
    });
}
