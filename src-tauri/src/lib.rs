mod theme;

use tauri::Manager;
use tauri_specta::{collect_commands, Builder};
use theme::ThemeState;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new()
        .commands(collect_commands![theme::set_theme, theme::get_theme,])
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
            let setting = theme::load_theme(app.handle());
            app.manage(ThemeState::new(setting));

            // ウィンドウを tauri.conf.json ではなくここで生成する。
            // 設定の読み込み後に作ることで、最初の描画からテーマが確定する。
            tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::default(),
            )
            .title("sample-tauri-app")
            .inner_size(800.0, 600.0)
            .theme(setting.to_window_theme())
            .initialization_script(theme::initialization_script(setting))
            .build()?;
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
        // tauri-specta は event / channel 用の定型コードを常に出力するため、
        // それらを使わない構成では noUnusedLocals に引っかかる。生成物は
        // 手で直せないのでヘッダーで型チェックから外す。
        specta_builder()
            .export(
                Typescript::default().header("// @ts-nocheck"),
                "../src/bindings.ts",
            )
            .expect("failed to export typescript bindings");
    }
}
