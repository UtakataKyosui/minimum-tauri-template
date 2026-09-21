use std::sync::Mutex;
use tauri::{Emitter, Manager};
use tauri_plugin_store::StoreExt;
mod theme;

struct ThemeState(Mutex<tauri::Theme>);

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(ThemeState(Mutex::new(tauri::Theme::Light)))
        .setup(|app | {
            let window = app.get_webview_window("main").unwrap();
            let store = app.store("settings.json").unwrap();
            if let Some(theme) = store.get("theme") {
                if let Ok(theme) = serde_json::from_value::<tauri::Theme>(theme) {
                    *app.state::<ThemeState>().inner().0.lock().unwrap() = theme;
                }
            }
            let value = &*app.state::<ThemeState>().inner().0.lock().unwrap();
            theme::apply_theme(&window, value.clone());

            window.clone().on_window_event(move |event| {
                if let tauri::WindowEvent::ThemeChanged(theme) = event {
                    window.emit("theme-changed", theme).unwrap();
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

