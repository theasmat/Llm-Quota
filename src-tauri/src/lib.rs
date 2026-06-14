pub mod commands;
pub mod constants;
pub mod error;
pub mod models;
pub mod modules;
#[cfg(test)]
mod tests;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let _ = modules::config::get_config_dir();
            
            // Set up tray icon and handlers
            let _ = modules::tray::setup_tray(app.handle());

            // Check if tray mode was saved in config
            let config = modules::config::load_config().unwrap_or_default();
            if config.tray_mode {
                modules::tray::set_tray_mode(app.handle(), true);
            }

            // Window blur listener to hide window if in tray mode
            use tauri::Manager;
            if let Some(window) = app.get_webview_window("main") {
                let app_handle = app.handle().clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(focused) = event {
                        if !focused {
                            let current_config = modules::config::load_config().unwrap_or_default();
                            if current_config.tray_mode {
                                let _ = app_handle.get_webview_window("main").unwrap().hide();
                            }
                        }
                    }
                });
            }

            Ok(())
        });

    builder
        .invoke_handler(commands::get_handlers())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
