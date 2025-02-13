use tauri::{
    menu::{MenuBuilder, MenuItem},
    tray::TrayIconBuilder,
    Manager, 
    Runtime,
};
use tauri_plugin_positioner::WindowExt;



#[allow(dead_code)]
pub fn create_tray<R: Runtime>( app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let copilot_item   = MenuItem::with_id(app, "copilot",   "Copilot",   false, None::<&str>)?;
    let show_hide_item = MenuItem::with_id(app, "show_hide", "Hide",      true,  None::<&str>)?;
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
                //tauri_plugin_positioner::on_tray_event(app.app_handle(), &event);
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        if let Some(item) = menu.get(&event.id) {
                            let _ = item.as_menuitem().and_then(|item| Some(item.set_text("Show")));
                        }
                        let _ = window.hide();
                    } else {
                        // Change the menu item text to "Hide"
                        if let Some(item) = menu.get(&event.id) {
                            let _ = item.as_menuitem().and_then(|item| Some(item.set_text("Hide")));
                        }
                        let _ = window.move_window(tauri_plugin_positioner::Position::TopRight);
                        let _ = window.set_always_on_top(true);
                        let _ = window.set_focus();
                        let _ = window.show();
                    }
                }
            }
            
            _ => {}
        })
        .build(app);
    Ok(())
}
