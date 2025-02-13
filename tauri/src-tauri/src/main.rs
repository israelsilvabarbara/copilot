// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{menu::{MenuBuilder, MenuItem}, tray::TrayIconBuilder, Manager};
use tauri_plugin_positioner::{WindowExt, Position};

fn main() {
    copilot_lib::run()
    
    
}

