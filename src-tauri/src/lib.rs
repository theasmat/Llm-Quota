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
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|_app| {
            let _ = modules::config::get_config_dir();
            Ok(())
        });

    builder
        .invoke_handler(commands::get_handlers())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
