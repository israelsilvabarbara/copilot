use tauri::{
    menu::{MenuBuilder, MenuItem}, Manager, Runtime
};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};


use crate::position::WindowPosition;


#[allow(dead_code)]
pub fn create_tray<R: Runtime>( app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let copilot_item   = MenuItem::with_id(app, "copilot",   "Copilot",   false, None::<&str>)?;
    let show_hide_item = MenuItem::with_id(app, "show_hide", "Show/Hide", true,  None::<&str>)?;
    let quit_item      = MenuItem::with_id(app, "quit",      "Quit",      true,  None::<&str>)?;
    
    let menu = MenuBuilder::new(app)
                    .item(&copilot_item)
                    .separator()
                    .item(&show_hide_item)
                    .separator()
                    .item(&quit_item)
                    .build()?;
    
    let _ = TrayIconBuilder::with_id("tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "show_hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        // Save the current position before hiding
                        if let Ok(position) = window.outer_position() {
                            app.state::<WindowPosition>().set(Some(position));
                        }
                        let _ = window.hide();
                        println!("hide");
                    } else {
                        // Restore the saved position when showing
                        if let Some(position) = app.state::<WindowPosition>().get() {
                            let _ = window.set_position(position);
                        }
                        let _ = window.show();
                        let _ = window.set_focus();
                        println!("show");
                    }
                }
            }
            _ => {}
        })
        .on_tray_icon_event( |tray, event| {
            println!("event: {:#?}", event);
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if  !window.is_visible().unwrap_or(false) {
                        let _ = window.show();
                        let _ = window.set_focus();
                        println!("show");
                    } else {
                        let _ = window.hide();
                        println!("hide");
                    }
                }
            }
        })
        .build(app);
    Ok(())
}



