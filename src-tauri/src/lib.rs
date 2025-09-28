// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod application;
use application::{event_handler::handle_menu_event, menu::create_menu};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn open_file_dialog(app_handle: tauri::AppHandle) {
    application::events::openfile::open_file(&app_handle);
}

#[tauri::command]
fn show_file_context_menu(window: tauri::Window, x: f64, y: f64) -> Result<(), String> {
    application::events::context_menu::show_for_file(&window, (x, y)).map_err(|err| err.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .menu(|app| create_menu(app))
        .on_menu_event(handle_menu_event)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            open_file_dialog,
            show_file_context_menu
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
