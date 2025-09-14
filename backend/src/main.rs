#[cfg(all(feature = "cli", feature = "tauri"))]
use parquet_studio::{
    application::{build_menu, create_file, handle_menu_event, open_file, FileApp},
    repositories::LocalFileRepository,
    services::ParquetService,
};
#[cfg(all(feature = "cli", feature = "tauri"))]
use tauri::Builder;

#[cfg(not(all(feature = "cli", feature = "tauri")))]
fn main() {}

#[cfg(all(feature = "cli", feature = "tauri"))]
fn main() {
    Builder::default()
        .manage(FileApp::new(ParquetService::new(LocalFileRepository)))
        .invoke_handler(tauri::generate_handler![open_file, create_file])
        .setup(|app| {
            let menu = build_menu(&app.handle());
            app.set_menu(menu);
            Ok(())
        })
        .on_menu_event(|app, event| handle_menu_event(app, event))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
