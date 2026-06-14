pub mod macos;
pub mod windows;
pub mod linux;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, LogicalPosition,
};

pub fn set_tray_mode(app: &AppHandle, enabled: bool) {
    #[cfg(target_os = "macos")]
    macos::set_tray_mode(app, enabled);

    #[cfg(target_os = "windows")]
    windows::set_tray_mode(app, enabled);

    #[cfg(target_os = "linux")]
    linux::set_tray_mode(app, enabled);
}

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit Llm Quota", true, None::<&str>)?;
    let settings_i = MenuItem::with_id(app, "settings", "Open Settings", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&settings_i, &quit_i])?;

    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "settings" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                    let _ = window.eval("window.location.href = '/settings'");
                }
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                rect,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    // Try to calculate position relative to tray icon
                    let win_size = window.outer_size().unwrap_or_default();
                    let scale_factor = window.scale_factor().unwrap_or(1.0);
                    let win_logical_size = win_size.to_logical::<f64>(scale_factor);
                    
                    let (rect_x, rect_y) = match rect.position {
                        tauri::Position::Physical(p) => (p.x as f64 / scale_factor, p.y as f64 / scale_factor),
                        tauri::Position::Logical(p) => (p.x, p.y),
                    };
                    let (rect_w, rect_h) = match rect.size {
                        tauri::Size::Physical(s) => (s.width as f64 / scale_factor, s.height as f64 / scale_factor),
                        tauri::Size::Logical(s) => (s.width, s.height),
                    };
                    
                    let mut x = rect_x - (win_logical_size.width / 2.0) + (rect_w / 2.0);
                    
                    #[cfg(target_os = "macos")]
                    let y = rect_y + rect_h;
                    
                    #[cfg(not(target_os = "macos"))]
                    let y = rect_y - win_logical_size.height;
                    
                    // Simple bounds check (avoid negative x)
                    if x < 0.0 { x = 0.0; }

                    let _ = window.set_position(LogicalPosition::new(x, y));
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}
