mod theme;

use tauri::Manager;
use theme::ThemeState;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let setting = theme::load_theme(app.handle());
            app.manage(ThemeState::new(setting));

            let window = app
                .get_webview_window("main")
                .expect("main window not found");
            theme::apply_theme(&window, setting);
            theme::handle_theme_changed(&window);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            theme::set_theme,
            theme::get_theme,
            theme::get_resolved_theme
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
