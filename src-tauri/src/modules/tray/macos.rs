#[cfg(target_os = "macos")]
use tauri::{AppHandle, Manager};

#[cfg(target_os = "macos")]
pub fn set_tray_mode(app: &AppHandle, enabled: bool) {
    // macOS: Use ActivationPolicy to hide/show from dock
    if enabled {
        let _ = app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    } else {
        let _ = app.set_activation_policy(tauri::ActivationPolicy::Regular);
    }
}
