#![cfg(not(target_arch = "wasm32"))]

use parquetstudio_lib::application::event_handler::{
    resolve_menu_event_action, MenuEventAction,
};

#[test]
fn given_file_open_event_when_resolve_menu_event_action_then_should_return_open_file() {
    let action = resolve_menu_event_action("menu.file.open");

    assert_eq!(action, MenuEventAction::OpenFile);
}

#[test]
fn given_file_quit_event_when_resolve_menu_event_action_then_should_return_quit() {
    let action = resolve_menu_event_action("menu.file.quit");

    assert_eq!(action, MenuEventAction::Quit);
}

#[test]
fn given_unknown_event_when_resolve_menu_event_action_then_should_return_none() {
    let action = resolve_menu_event_action("menu.unknown");

    assert_eq!(action, MenuEventAction::None);
}
