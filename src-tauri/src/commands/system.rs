use crate::modules;

#[tauri::command]
pub fn get_data_dir_path(_app_handle: tauri::AppHandle) -> Result<String, String> {
    Ok(modules::config::get_config_dir()
        .to_string_lossy()
        .into_owned())
}

#[tauri::command]
pub fn read_text_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_text_file(path: String, content: String) -> Result<(), String> {
    std::fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn check_proxy_latency(_proxy_id: String) -> Result<i32, String> {
    Ok(0)
}

#[tauri::command]
pub async fn show_main_window(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::Manager;
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
    Ok(())
}

#[tauri::command]
pub async fn set_window_theme(app: tauri::AppHandle, theme: String) -> Result<(), String> {
    use tauri::Manager;
    if let Some(window) = app.get_webview_window("main") {
        let tauri_theme = match theme.as_str() {
            "dark" => Some(tauri::Theme::Dark),
            "light" => Some(tauri::Theme::Light),
            _ => None,
        };
        let _ = window.set_theme(tauri_theme);
    }
    Ok(())
}

#[tauri::command]
pub async fn set_tray_mode(app: tauri::AppHandle, enabled: bool) -> Result<(), String> {
    crate::modules::tray::set_tray_mode(&app, enabled);
    Ok(())
}
