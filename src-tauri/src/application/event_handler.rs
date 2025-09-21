use super::events::openfile::open_file;
use tauri::menu::MenuEvent;
use tauri::{AppHandle, Runtime};

pub fn handle_menu_event<R: Runtime>(app_handle: &AppHandle<R>, event: MenuEvent) {
    println!("Handling menu event: {}", event.id().as_ref());
    match event.id().as_ref() {
        "menu.file.open" => {
            open_file(app_handle);
        }
        "menu.file.quit" => {
            app_handle.exit(0);
        }
        _ => {}
    }
}
