#[cfg(target_os = "macos")]
use tauri::AppHandle;

#[cfg(target_os = "macos")]
pub fn set_tray_mode(app: &AppHandle, enabled: bool) {
    // macOS: Use ActivationPolicy to hide/show from dock
    if enabled {
        if let Err(e) = app.set_activation_policy(tauri::ActivationPolicy::Accessory) {
            eprintln!("Failed to set accessory policy: {}", e);
        }
    } else {
        if let Err(e) = app.set_activation_policy(tauri::ActivationPolicy::Regular) {
            eprintln!("Failed to set regular policy: {}", e);
        }
    }
}
