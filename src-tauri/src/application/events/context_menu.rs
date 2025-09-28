use tauri::menu::{ContextMenu, IsMenuItem, Menu, MenuItem, Submenu};
use tauri::{Runtime, Window};

pub const FILE_CONTEXT_NEW_QUERY_ID: &str = "context.file.new_query";
pub const FILE_CONTEXT_SCRIPTS_MENU_ID: &str = "context.file.scripts";
pub const FILE_CONTEXT_SCRIPTS_SELECT_ID: &str = "context.file.scripts.select";
pub const FILE_CONTEXT_SCRIPTS_UPDATE_ID: &str = "context.file.scripts.update";
pub const FILE_CONTEXT_SCRIPTS_INSERT_ID: &str = "context.file.scripts.insert";
pub const FILE_CONTEXT_SCRIPTS_DELETE_ID: &str = "context.file.scripts.delete";

pub fn show_for_file<R: Runtime>(window: &Window<R>, _position: (f64, f64)) -> tauri::Result<()> {
    let new_query = MenuItem::with_id(
        window,
        FILE_CONTEXT_NEW_QUERY_ID,
        "New query",
        true,
        None::<&str>,
    )?;

    let script_select = MenuItem::with_id(
        window,
        FILE_CONTEXT_SCRIPTS_SELECT_ID,
        "SELECT",
        true,
        None::<&str>,
    )?;
    let script_update = MenuItem::with_id(
        window,
        FILE_CONTEXT_SCRIPTS_UPDATE_ID,
        "UPDATE",
        true,
        None::<&str>,
    )?;
    let script_insert = MenuItem::with_id(
        window,
        FILE_CONTEXT_SCRIPTS_INSERT_ID,
        "INSERT",
        true,
        None::<&str>,
    )?;
    let script_delete = MenuItem::with_id(
        window,
        FILE_CONTEXT_SCRIPTS_DELETE_ID,
        "DELETE",
        true,
        None::<&str>,
    )?;

    let script_items: Vec<&dyn IsMenuItem<R>> = vec![
        &script_select,
        &script_update,
        &script_insert,
        &script_delete,
    ];
    let scripts_submenu = Submenu::with_id_and_items(
        window,
        FILE_CONTEXT_SCRIPTS_MENU_ID,
        "Scripts",
        true,
        &script_items,
    )?;

    let menu_items: Vec<&dyn IsMenuItem<R>> = vec![&new_query, &scripts_submenu];
    let menu = Menu::with_items(window, &menu_items)?;

    menu.popup(window.clone())
}
