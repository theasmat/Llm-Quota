#[cfg(target_os = "linux")]
use tauri::{AppHandle, Manager};

#[cfg(target_os = "linux")]
pub fn set_tray_mode(app: &AppHandle, enabled: bool) {
    // Linux: Try to use skip_taskbar if supported by the window manager
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_skip_taskbar(enabled);
    }
}
