use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Manager};

use crate::{
    enums::{MenuItemId, MenuItemTitle, WindowId},
    state::AppState,
};

pub fn toggle_always_on_top(app: &AppHandle, state: Arc<Mutex<AppState>>) {
    let always_on_top_handle = app.tray_handle().get_item(MenuItemId::AlwaysOnTop.as_str());
    let window = app.get_window(WindowId::Main.as_str()).unwrap();
    let opposite = !state.lock().unwrap().always_on_top;

    window.set_always_on_top(opposite).unwrap();
    always_on_top_handle.set_selected(opposite).unwrap();
    state.lock().unwrap().always_on_top = opposite;
}

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
