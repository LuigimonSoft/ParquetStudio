use tauri::{AppHandle, Emitter, Manager, Runtime};
use tauri_plugin_dialog::{DialogExt, FilePath};

pub fn open_file<R: Runtime>(app_handle: &AppHandle<R>) {
    let mut dialog = app_handle
        .dialog()
        .file()
        .set_title("Select parquet file")
        .add_filter("Parquet files", &["parquet"]);

    if let Some(window) = app_handle.get_webview_window("main") {
        dialog = dialog.set_parent(&window);
    }

    let handle = app_handle.clone();
    dialog.pick_file(move |selection| {
        if let Some(selection) = selection {
            if let Err(err) = handle.emit(
                "menu://files/opened",
                match selection {
                    FilePath::Path(path) => path.to_string_lossy().to_string(),
                    FilePath::Url(url) => url.to_string(),
                },
            ) {
                eprintln!("Failed to emit event: {}", err);
            }
        }
    });
}
