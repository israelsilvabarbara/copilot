use tauri::PhysicalPosition;
use std::sync::Mutex;

pub struct WindowPosition {
    position: Mutex<Option<PhysicalPosition<i32>>>,
}

impl WindowPosition {
    pub fn set(&self, position: Option<PhysicalPosition<i32>>) {
        let mut pos = self.position.lock().unwrap();
        *pos = position;
    }

    pub fn get(&self) -> Option<PhysicalPosition<i32>> {
        let pos = self.position.lock().unwrap();
        *pos
    }
}

impl Default for WindowPosition {
    fn default() -> Self {
        WindowPosition {
            position: Mutex::new(None),
        }
    }
}
