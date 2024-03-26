use tauri::{AppHandle, Manager};

use crate::enums::{MenuItemId, MenuItemTitle, WindowId};

pub fn toggle_visibility(app: &AppHandle) {
    let toggle_handle = app.tray_handle().get_item(MenuItemId::Toggle.as_str());
    let window = app.get_window(WindowId::Main.as_str()).unwrap();

    if window.is_visible().unwrap() {
        window.hide().unwrap();
        toggle_handle
            .set_title(MenuItemTitle::Show.as_str())
            .unwrap();
    } else {
        window.show().unwrap();
        toggle_handle
            .set_title(MenuItemTitle::Hide.as_str())
            .unwrap();
    }
}

pub fn quit(app: &AppHandle) {
    app.exit(0);
}
