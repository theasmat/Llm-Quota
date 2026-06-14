#[cfg(target_os = "windows")]
use tauri::{AppHandle, Manager};

#[cfg(target_os = "windows")]
pub fn set_tray_mode(app: &AppHandle, enabled: bool) {
    // Windows: Use set_skip_taskbar to hide/show from taskbar
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_skip_taskbar(enabled);
    }
}
