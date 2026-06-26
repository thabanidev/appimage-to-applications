mod commands;
mod linux;
mod models;

use commands::{
    install_appimage, list_installed_apps, parse_app_name, preview_install,
    remove_installed_app, suggest_app_category, update_installed_app,
};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    const APP_TITLE: &str = "AppImage to Applications";

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                if let Some(icon) = app.default_window_icon().cloned() {
                    let _ = window.set_icon(icon);
                }

                let _ = window.set_title(APP_TITLE);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            parse_app_name,
            suggest_app_category,
            preview_install,
            install_appimage,
            list_installed_apps,
            remove_installed_app,
            update_installed_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
