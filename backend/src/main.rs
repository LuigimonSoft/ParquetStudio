#[cfg(feature = "cli")]
use parquet_studio::application::{build_menu, handle_menu_event};
#[cfg(feature = "cli")]
use tauri::Builder;

#[cfg(not(feature = "cli"))]
fn main() {}

#[cfg(feature = "cli")]
fn main() {
    Builder::default()
        .setup(|app| {
            let menu = build_menu(&app.handle());
            app.set_menu(menu);
            Ok(())
        })
        .on_menu_event(|app, event| handle_menu_event(app, event))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
