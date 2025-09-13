use tauri::menu::{Menu, MenuBuilder, MenuEvent, MenuItemBuilder, SubmenuBuilder};
use tauri::{AppHandle, Emitter, Runtime};

/// Builds the application menu with basic and extra sections.
pub fn build_menu<R: Runtime>(app: &AppHandle<R>) -> Menu<R> {
    let open = MenuItemBuilder::with_id("open", "Open")
        .build(app)
        .expect("failed to build open item");
    let save = MenuItemBuilder::with_id("save", "Save")
        .build(app)
        .expect("failed to build save item");
    let file_menu = SubmenuBuilder::new(app, "File")
        .item(&open)
        .item(&save)
        .build()
        .expect("failed to build File menu");

    let create_file = MenuItemBuilder::with_id("create_file", "Create File")
        .build(app)
        .expect("failed to build create file item");
    let create_menu = SubmenuBuilder::new(app, "Create")
        .item(&create_file)
        .build()
        .expect("failed to build Create menu");

    let mcp_settings = MenuItemBuilder::with_id("mcp_settings", "MCP Settings")
        .build(app)
        .expect("failed to build MCP settings item");
    let mcp_menu = SubmenuBuilder::new(app, "MCP")
        .item(&mcp_settings)
        .build()
        .expect("failed to build MCP menu");

    MenuBuilder::new(app)
        .item(&file_menu)
        .item(&create_menu)
        .item(&mcp_menu)
        .build()
        .expect("failed to build menu")
}

/// Handles menu events and emits frontend events.
pub fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, event: MenuEvent) {
    match event.id.as_ref() {
        "open" => {
            let _ = app.emit("menu-open-file", ());
        }
        "save" => {
            let _ = app.emit("menu-save-file", ());
        }
        "create_file" => {
            let _ = app.emit("menu-create-file", ());
        }
        "mcp_settings" => {
            let _ = app.emit("menu-mcp-settings", ());
        }
        _ => {}
    }
}
