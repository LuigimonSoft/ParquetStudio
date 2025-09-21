use super::{events::openfile::open_file, menu::{FILE_OPEN_ID, FILE_QUIT_ID}};
use tauri::menu::MenuEvent;
use tauri::{AppHandle, Runtime};

pub fn handle_menu_event<R: Runtime>(app_handle: &AppHandle<R>, event: MenuEvent) {
    println!("Handling menu event: {}", event.id().as_ref());
    match event.id().as_ref() {
        FILE_OPEN_ID => {
            open_file(app_handle);
        }
        FILE_QUIT_ID => {
            app_handle.exit(0);
        }
        _ => {}
    }
}
