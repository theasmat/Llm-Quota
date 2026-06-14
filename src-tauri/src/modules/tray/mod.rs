pub mod macos;
pub mod windows;
pub mod linux;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, LogicalPosition, Emitter,
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
        .show_menu_on_left_click(false)
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
                if let Some(window) = app.get_webview_window("tray") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                        return;
                    }

                    let logical_size = window.inner_size().unwrap().to_logical::<f64>(window.scale_factor().unwrap());
                    let scale_factor = window.scale_factor().unwrap_or(1.0);
                    
                    let (rect_x, rect_y) = match rect.position {
                        tauri::Position::Physical(p) => (p.x as f64 / scale_factor, p.y as f64 / scale_factor),
                        tauri::Position::Logical(p) => (p.x, p.y),
                    };
                    let (_rect_w, rect_h) = match rect.size {
                        tauri::Size::Physical(s) => (s.width as f64 / scale_factor, s.height as f64 / scale_factor),
                        tauri::Size::Logical(s) => (s.width, s.height),
                    };
                    
                    let mut x = rect_x - (logical_size.width / 2.0);
                    let y = rect_y + rect_h + 4.0;
                    
                    if let Some(monitor) = window.current_monitor().unwrap() {
                        let monitor_size = monitor.size().to_logical::<f64>(window.scale_factor().unwrap());
                        if x + logical_size.width > monitor_size.width {
                            x = monitor_size.width - logical_size.width - 8.0;
                        }
                        if x < 0.0 { x = 0.0; }
                    }

                    let _ = window.set_position(LogicalPosition::new(x, y));
                    let _ = window.show();
                    let _ = window.set_focus();
                    let _ = window.emit("tray-opened", ());
                }
            }
        })
        .build(app)?;

    Ok(())
}
