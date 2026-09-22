mod theme;

use tauri::Manager;
use tauri_specta::{collect_commands, collect_events, Builder};
use theme::{ThemeChanged, ThemeState};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new()
        .commands(collect_commands![theme::set_theme, theme::get_theme,])
        .events(collect_events![ThemeChanged])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // bindings.ts の生成は `cargo test` (tests::export_bindings) が担う。
    // ここで生成すると同一ファイルへの書き込みが二重になる。
    let builder = specta_builder();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);

            let setting = theme::load_theme(app.handle());
            app.manage(ThemeState::new(setting));

            let window = app
                .get_webview_window("main")
                .expect("main window not found");
            theme::apply_theme(&window, setting);
            theme::handle_theme_changed(&window);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use specta_typescript::Typescript;

    /// アプリを起動せずに bindings を生成する。`cargo test` が生成の入口になる。
    #[test]
    fn export_bindings() {
        specta_builder()
            .export(Typescript::default(), "../src/bindings.ts")
            .expect("failed to export typescript bindings");
    }
}
