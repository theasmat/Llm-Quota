#[cfg(target_os = "macos")]
use tauri::AppHandle;

#[cfg(target_os = "macos")]
pub fn set_tray_mode(app: &AppHandle, enabled: bool) {
    let app_clone = app.clone();
    let _ = app.run_on_main_thread(move || {
        if enabled {
            // Hide the app to force macOS to drop the dock icon, change policy, then show
            let _ = app_clone.hide();
            if let Err(e) = app_clone.set_activation_policy(tauri::ActivationPolicy::Accessory) {
                eprintln!("Failed to set accessory policy: {}", e);
            }
            let _ = app_clone.show();
        } else {
            if let Err(e) = app_clone.set_activation_policy(tauri::ActivationPolicy::Regular) {
                eprintln!("Failed to set regular policy: {}", e);
            }
            let _ = app_clone.show();
        }
    });
}
