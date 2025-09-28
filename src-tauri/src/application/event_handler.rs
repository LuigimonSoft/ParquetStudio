use super::{
    events::{
        context_menu::{
            FILE_CONTEXT_NEW_QUERY_ID, FILE_CONTEXT_SCRIPTS_DELETE_ID,
            FILE_CONTEXT_SCRIPTS_INSERT_ID, FILE_CONTEXT_SCRIPTS_SELECT_ID,
            FILE_CONTEXT_SCRIPTS_UPDATE_ID,
        },
        openfile::open_file,
    },
    menu::{FILE_OPEN_ID, FILE_QUIT_ID},
};
use tauri::menu::MenuEvent;
use tauri::{AppHandle, Runtime};

#[derive(Debug, PartialEq, Eq)]
pub enum MenuEventAction<'a> {
    OpenFile,
    Quit,
    Context(&'a str),
    None,
}

/// Determines the action that should be taken for the provided menu event ID.
pub fn resolve_menu_event_action(event_id: &str) -> MenuEventAction<'_> {
    match event_id {
        FILE_OPEN_ID => MenuEventAction::OpenFile,
        FILE_QUIT_ID => MenuEventAction::Quit,
        FILE_CONTEXT_NEW_QUERY_ID
        | FILE_CONTEXT_SCRIPTS_SELECT_ID
        | FILE_CONTEXT_SCRIPTS_UPDATE_ID
        | FILE_CONTEXT_SCRIPTS_INSERT_ID
        | FILE_CONTEXT_SCRIPTS_DELETE_ID => MenuEventAction::Context(event_id),
        _ => MenuEventAction::None,
    }
}

pub fn handle_menu_event<R: Runtime>(app_handle: &AppHandle<R>, event: MenuEvent) {
    println!("Handling menu event: {}", event.id().as_ref());
    match resolve_menu_event_action(event.id().as_ref()) {
        MenuEventAction::OpenFile => {
            open_file(app_handle);
        }
        MenuEventAction::Quit => {
            app_handle.exit(0);
        }
        MenuEventAction::Context(id) => {
            println!("Context menu action: {}", id);
        }
        MenuEventAction::None => {}
    }
}
