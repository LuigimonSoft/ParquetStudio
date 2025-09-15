#![cfg(feature = "tauri")]

use parquet_studio::application::build_menu;
use tauri::menu::MenuItemKind;
use tauri::test::mock_app;

#[test]
fn given_build_menu_when_called_then_should_include_basic_and_extra_menus() {
    let app = mock_app();
    let menu = build_menu(&app.handle());
    let labels: Vec<String> = menu
        .items()
        .unwrap()
        .into_iter()
        .filter_map(|entry| match entry {
            MenuItemKind::Submenu(sub) => sub.text().ok(),
            _ => None,
        })
        .collect();
    assert!(labels.contains(&"File".to_string()));
    assert!(labels.contains(&"Create".to_string()));
    assert!(labels.contains(&"MCP".to_string()));
}
