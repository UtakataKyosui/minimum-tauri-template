use crate::{ThemeState};
use tauri::{Emitter, window};
use tauri_plugin_store::StoreExt;

pub fn apply_theme(window: &tauri::WebviewWindow, theme: tauri::Theme) -> tauri::Theme {
    window.set_theme(Some(theme)).unwrap();
    window.theme().unwrap_or(tauri::Theme::Light)
}

#[tauri::command]
pub fn set_theme(theme: tauri::Theme,window: &tauri::WebviewWindow,state: tauri::State<ThemeState>) {
    *state.inner().0.lock().unwrap() = theme.clone();
    apply_theme(window, theme);
    window.emit("theme-changed", theme.clone()).expect("theme emit error");
}

pub fn save_theme(app: &tauri::AppHandle, theme: tauri::Theme) {
    let store = app.store("settings.json").unwrap();
    store.set("theme", serde_json::to_value(theme).unwrap());
    store.save().unwrap();
}

pub fn load_theme(app: &tauri::AppHandle) -> tauri::Theme {
    let store = app.store("settings.json").unwrap();
    if let Some(theme) = store.get("theme") {
        if let Ok(theme) = serde_json::from_value::<tauri::Theme>(theme) {
            return theme;
        }
    }
    tauri::Theme::Light
}
    
fn handle_theme_changed(window: &tauri::WebviewWindow) -> Result<(), tauri::Error> {
    window.clone().on_window_event(move | event | {
        if let tauri::WindowEvent::ThemeChanged(ref theme) = event {
            window.emit("theme-changed", theme.clone());
        } 
    });
}