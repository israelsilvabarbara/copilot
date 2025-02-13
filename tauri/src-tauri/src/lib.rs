use tauri::Manager;
use tauri_plugin_positioner::{Position, WindowExt};


pub mod tray;
pub mod position;

//#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            if let Some(window)  = app.get_webview_window("main") {
                let primary_monitor = window.primary_monitor().unwrap().unwrap();
                let screen_height = primary_monitor.size().height;
    
                window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                    width: 400,
                    height: screen_height,
                })).unwrap(); 
                window.move_window(Position::TopRight).unwrap();
            }

            tray::create_tray(app.handle())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
