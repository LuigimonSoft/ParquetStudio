#[cfg(target_os = "macos")]
use tauri::menu::AboutMetadata;
use tauri::menu::{IsMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::{AppHandle, Runtime};

const FILE_MENU_ID: &str = "menu.file";
const FILE_NEW_ID: &str = "menu.file.new";
pub const FILE_OPEN_ID: &str = "menu.file.open";
const FILE_SAVE_ID: &str = "menu.file.save";
const FILE_SAVE_AS_ID: &str = "menu.file.save_as";
const FILE_CLOSE_ID: &str = "menu.file.close";
pub const FILE_QUIT_ID: &str = "menu.file.quit";

const EDIT_MENU_ID: &str = "menu.edit";
const EDIT_UNDO_ID: &str = "menu.edit.undo";
const EDIT_REDO_ID: &str = "menu.edit.redo";
const EDIT_CUT_ID: &str = "menu.edit.cut";
const EDIT_COPY_ID: &str = "menu.edit.copy";
const EDIT_PASTE_ID: &str = "menu.edit.paste";
const EDIT_SELECT_ALL_ID: &str = "menu.edit.select_all";

#[cfg(target_os = "macos")]
const APP_MENU_ID: &str = "menu.app";
#[cfg(target_os = "macos")]
const APP_PREFERENCES_ID: &str = "menu.app.preferences";

const VIEW_MENU_ID: &str = "menu.view";
const VIEW_RELOAD_ID: &str = "menu.view.reload";
const VIEW_RESET_ZOOM_ID: &str = "menu.view.reset_zoom";
const VIEW_ZOOM_IN_ID: &str = "menu.view.zoom_in";
const VIEW_ZOOM_OUT_ID: &str = "menu.view.zoom_out";
const VIEW_FULL_SCREEN_ID: &str = "menu.view.full_screen";
const VIEW_DEVTOOLS_ID: &str = "menu.view.toggle_devtools";

const WINDOW_MENU_ID: &str = "menu.window";
const WINDOW_MINIMIZE_ID: &str = "menu.window.minimize";
const WINDOW_ZOOM_ID: &str = "menu.window.zoom";
const WINDOW_CLOSE_ID: &str = "menu.window.close";
const WINDOW_ALL_FRONT_ID: &str = "menu.window.all_to_front";

const HELP_MENU_ID: &str = "menu.help";
const HELP_LEARN_MORE_ID: &str = "menu.help.learn_more";
const HELP_DOCUMENTATION_ID: &str = "menu.help.documentation";
const HELP_REPORT_ISSUE_ID: &str = "menu.help.report_issue";
const HELP_ABOUT_ID: &str = "menu.help.about";

pub fn create_menu<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let file_new = MenuItem::with_id(app, FILE_NEW_ID, "New", true, Some("CmdOrCtrl+N"))?;
    let file_open = MenuItem::with_id(app, FILE_OPEN_ID, "Open...", true, Some("CmdOrCtrl+O"))?;
    let file_save = MenuItem::with_id(app, FILE_SAVE_ID, "Save", true, Some("CmdOrCtrl+S"))?;
    let file_save_as = MenuItem::with_id(
        app,
        FILE_SAVE_AS_ID,
        "Save As...",
        true,
        Some("Shift+CmdOrCtrl+S"),
    )?;
    let file_close = MenuItem::with_id(
        app,
        FILE_CLOSE_ID,
        "Close Window",
        true,
        Some("CmdOrCtrl+W"),
    )?;
    let file_quit = MenuItem::with_id(app, FILE_QUIT_ID, "Quit", true, Some("CmdOrCtrl+Q"))?;
    let file_separator = PredefinedMenuItem::separator(app)?;

    let file_items: Vec<&dyn IsMenuItem<R>> = vec![
        &file_new,
        &file_open,
        &file_save,
        &file_save_as,
        &file_separator,
        &file_close,
        &file_quit,
    ];

    let file_menu = Submenu::with_id_and_items(app, FILE_MENU_ID, "File", true, &file_items)?;

    let edit_undo = MenuItem::with_id(app, EDIT_UNDO_ID, "Undo", true, Some("CmdOrCtrl+Z"))?;
    let edit_redo = MenuItem::with_id(app, EDIT_REDO_ID, "Redo", true, Some("Shift+CmdOrCtrl+Z"))?;
    let edit_cut = MenuItem::with_id(app, EDIT_CUT_ID, "Cut", true, Some("CmdOrCtrl+X"))?;
    let edit_copy = MenuItem::with_id(app, EDIT_COPY_ID, "Copy", true, Some("CmdOrCtrl+C"))?;
    let edit_paste = MenuItem::with_id(app, EDIT_PASTE_ID, "Paste", true, Some("CmdOrCtrl+V"))?;
    let edit_select_all = MenuItem::with_id(
        app,
        EDIT_SELECT_ALL_ID,
        "Select All",
        true,
        Some("CmdOrCtrl+A"),
    )?;
    let edit_separator = PredefinedMenuItem::separator(app)?;

    let edit_items: Vec<&dyn IsMenuItem<R>> = vec![
        &edit_undo,
        &edit_redo,
        &edit_separator,
        &edit_cut,
        &edit_copy,
        &edit_paste,
        &edit_select_all,
    ];

    let edit_menu = Submenu::with_id_and_items(app, EDIT_MENU_ID, "Edit", true, &edit_items)?;

    let view_reload = MenuItem::with_id(app, VIEW_RELOAD_ID, "Reload", true, Some("CmdOrCtrl+R"))?;
    let view_reset_zoom = MenuItem::with_id(
        app,
        VIEW_RESET_ZOOM_ID,
        "Reset Zoom",
        true,
        Some("CmdOrCtrl+0"),
    )?;
    let view_zoom_in =
        MenuItem::with_id(app, VIEW_ZOOM_IN_ID, "Zoom In", true, Some("CmdOrCtrl+="))?;
    let view_zoom_out =
        MenuItem::with_id(app, VIEW_ZOOM_OUT_ID, "Zoom Out", true, Some("CmdOrCtrl+-"))?;
    let view_full_screen = MenuItem::with_id(
        app,
        VIEW_FULL_SCREEN_ID,
        "Toggle Full Screen",
        true,
        Some("F11"),
    )?;
    let view_devtools = MenuItem::with_id(
        app,
        VIEW_DEVTOOLS_ID,
        "Toggle Developer Tools",
        true,
        Some("Alt+CmdOrCtrl+I"),
    )?;
    let view_separator_zoom = PredefinedMenuItem::separator(app)?;
    let view_separator_tools = PredefinedMenuItem::separator(app)?;

    let view_items: Vec<&dyn IsMenuItem<R>> = vec![
        &view_reload,
        &view_separator_zoom,
        &view_reset_zoom,
        &view_zoom_in,
        &view_zoom_out,
        &view_separator_tools,
        &view_full_screen,
        &view_devtools,
    ];

    let view_menu = Submenu::with_id_and_items(app, VIEW_MENU_ID, "View", true, &view_items)?;

    let window_minimize = MenuItem::with_id(
        app,
        WINDOW_MINIMIZE_ID,
        "Minimize",
        true,
        Some("CmdOrCtrl+M"),
    )?;
    let window_zoom = MenuItem::with_id(app, WINDOW_ZOOM_ID, "Zoom", true, None::<&str>)?;
    let window_close = MenuItem::with_id(
        app,
        WINDOW_CLOSE_ID,
        "Close",
        true,
        Some("CmdOrCtrl+Shift+W"),
    )?;
    let window_separator = PredefinedMenuItem::separator(app)?;
    let window_all_front = MenuItem::with_id(
        app,
        WINDOW_ALL_FRONT_ID,
        "Bring All to Front",
        true,
        None::<&str>,
    )?;

    let window_items: Vec<&dyn IsMenuItem<R>> = vec![
        &window_minimize,
        &window_zoom,
        &window_close,
        &window_separator,
        &window_all_front,
    ];

    let window_menu =
        Submenu::with_id_and_items(app, WINDOW_MENU_ID, "Window", true, &window_items)?;

    let help_learn_more =
        MenuItem::with_id(app, HELP_LEARN_MORE_ID, "Learn More", true, Some("F1"))?;
    let help_documentation = MenuItem::with_id(
        app,
        HELP_DOCUMENTATION_ID,
        "Documentation",
        true,
        None::<&str>,
    )?;
    let help_report_issue = MenuItem::with_id(
        app,
        HELP_REPORT_ISSUE_ID,
        "Report an Issue",
        true,
        None::<&str>,
    )?;
    let help_separator = PredefinedMenuItem::separator(app)?;
    let help_about = MenuItem::with_id(app, HELP_ABOUT_ID, "About", true, None::<&str>)?;

    let help_items: Vec<&dyn IsMenuItem<R>> = vec![
        &help_learn_more,
        &help_documentation,
        &help_report_issue,
        &help_separator,
        &help_about,
    ];

    let help_menu = Submenu::with_id_and_items(app, HELP_MENU_ID, "Help", true, &help_items)?;

    #[cfg(target_os = "macos")]
    let app_menu = {
        let pkg_info = app.package_info();
        let config = app.config();

        let about_metadata = AboutMetadata {
            name: Some(pkg_info.name.clone()),
            version: Some(pkg_info.version.to_string()),
            copyright: config.bundle.copyright.clone(),
            authors: config
                .bundle
                .publisher
                .clone()
                .map(|publisher| vec![publisher]),
            ..Default::default()
        };

        let about_label = format!("About {}", pkg_info.name);

        let app_about =
            PredefinedMenuItem::about(app, Some(about_label.as_str()), Some(about_metadata))?;
        let app_separator_about = PredefinedMenuItem::separator(app)?;
        let app_preferences = MenuItem::with_id(
            app,
            APP_PREFERENCES_ID,
            "Preferences...",
            true,
            Some("CmdOrCtrl+,"),
        )?;
        let app_services = PredefinedMenuItem::services(app, None)?;
        let app_separator_services = PredefinedMenuItem::separator(app)?;
        let app_hide = PredefinedMenuItem::hide(app, Some("Hide"))?;
        let app_hide_others = PredefinedMenuItem::hide_others(app, Some("Hide Others"))?;
        let app_show_all = PredefinedMenuItem::show_all(app, Some("Show All"))?;
        let app_separator_quit = PredefinedMenuItem::separator(app)?;
        let app_quit = PredefinedMenuItem::quit(app, None)?;

        let app_items: Vec<&dyn IsMenuItem<R>> = vec![
            &app_about,
            &app_separator_about,
            &app_preferences,
            &app_services,
            &app_separator_services,
            &app_hide,
            &app_hide_others,
            &app_show_all,
            &app_separator_quit,
            &app_quit,
        ];

        Submenu::with_id_and_items(app, APP_MENU_ID, pkg_info.name.clone(), true, &app_items)?
    };

    let mut menu_items: Vec<&dyn IsMenuItem<R>> = Vec::new();

    #[cfg(target_os = "macos")]
    menu_items.push(&app_menu);

    menu_items.push(&file_menu);
    menu_items.push(&edit_menu);
    menu_items.push(&view_menu);
    menu_items.push(&window_menu);
    menu_items.push(&help_menu);

    Menu::with_items(app, &menu_items)
}
