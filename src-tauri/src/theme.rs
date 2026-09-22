use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_store::StoreExt;

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

    fn as_str(self) -> &'static str {
        match self {
            ThemeSetting::System => "system",
            ThemeSetting::Light => "light",
            ThemeSetting::Dark => "dark",
        }
    }
}

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
    let setting = setting.as_str();
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

/// 設定を保存し、ウィンドウの外観へ反映する。
/// 実際に適用されたテーマはフロントが webview の prefers-color-scheme から
/// 読むため、ここでは返さない。ウィンドウへ問い合わせると、外観の反映前に
/// 答えが返って System のとき古い値を掴む。
#[tauri::command]
#[specta::specta]
pub fn set_theme(
    setting: ThemeSetting,
    window: tauri::WebviewWindow,
    state: tauri::State<ThemeState>,
) {
    state.set(setting);
    let _ = window.set_theme(setting.to_window_theme());
    save_theme(window.app_handle(), setting);
}

#[tauri::command]
#[specta::specta]
pub fn get_theme(state: tauri::State<ThemeState>) -> ThemeSetting {
    state.get()
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
