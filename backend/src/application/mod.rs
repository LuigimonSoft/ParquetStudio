mod file_app;

#[cfg(feature = "tauri")]
mod menu;

pub use file_app::FileApp;

#[cfg(feature = "tauri")]
pub use file_app::{create_file, open_file};
#[cfg(feature = "tauri")]
pub use menu::{build_menu, handle_menu_event};
