use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{Emitter, Manager};
use tauri_plugin_store::StoreExt;

const STORE_FILE: &str = "settings.json";
const STORE_KEY: &str = "theme";
pub const THEME_CHANGED_EVENT: &str = "theme-changed";

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
    fn to_window_theme(self) -> Option<tauri::Theme> {
        match self {
            ThemeSetting::System => None,
            ThemeSetting::Light => Some(tauri::Theme::Light),
            ThemeSetting::Dark => Some(tauri::Theme::Dark),
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

/// 設定をウィンドウへ反映し、実際に適用されたテーマを返す。
pub fn apply_theme(window: &tauri::WebviewWindow, setting: ThemeSetting) -> tauri::Theme {
    let _ = window.set_theme(setting.to_window_theme());
    window.theme().unwrap_or(tauri::Theme::Light)
}

#[tauri::command]
#[specta::specta]
pub fn set_theme(
    setting: ThemeSetting,
    window: tauri::WebviewWindow,
    state: tauri::State<ThemeState>,
) -> tauri::Theme {
    state.set(setting);
    let resolved = apply_theme(&window, setting);
    save_theme(window.app_handle(), setting);
    let _ = window.emit(THEME_CHANGED_EVENT, resolved);
    resolved
}

#[tauri::command]
#[specta::specta]
pub fn get_theme(state: tauri::State<ThemeState>) -> ThemeSetting {
    state.get()
}

/// 設定値と、それを解決した実際のテーマの両方を返す。
/// System のとき、フロントは起動直後の解決値をこれで知る。
#[tauri::command]
#[specta::specta]
pub fn get_resolved_theme(
    window: tauri::WebviewWindow,
    state: tauri::State<ThemeState>,
) -> (ThemeSetting, tauri::Theme) {
    (
        state.get(),
        window.theme().unwrap_or(tauri::Theme::Light),
    )
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
/// 明示指定中は set_theme で固定済みなのでこのイベントは発火しない。
pub fn handle_theme_changed(window: &tauri::WebviewWindow) {
    let emitter = window.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::ThemeChanged(theme) = event {
            if let Some(state) = emitter.try_state::<ThemeState>() {
                if state.get() == ThemeSetting::System {
                    let _ = emitter.emit(THEME_CHANGED_EVENT, *theme);
                }
            }
        }
    });
}
