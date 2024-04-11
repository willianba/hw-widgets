use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Manager, WindowBuilder, WindowUrl};

use crate::{
    enums::{MenuItemId, MenuItemTitle, WindowId, WindowTitle},
    AppState,
};

pub fn open_settings(app: &AppHandle, state: Arc<Mutex<AppState>>) {
    match app.get_window(WindowId::Settings.as_str()) {
        Some(settings_window) => {
            settings_window.set_focus().unwrap();
            return;
        }
        None => {
            WindowBuilder::new(app, WindowId::Settings.as_str(), WindowUrl::App("settings".into()))
                .title(WindowTitle::Settings.as_str())
                .always_on_top(state.lock().unwrap().always_on_top)
                .center()
                .build()
                .expect("Failed to create settings window");
        }
    }
}

pub fn toggle_always_on_top(app: &AppHandle, state: Arc<Mutex<AppState>>) {
    let always_on_top_handle = app.tray_handle().get_item(MenuItemId::AlwaysOnTop.as_str());
    let main_window = app.get_window(WindowId::Main.as_str()).unwrap();
    let opposite = !state.lock().unwrap().always_on_top;

    // If the settings window is open, set it to always on top too
    match app.get_window(WindowId::Settings.as_str()) {
        Some(settings_window) => {
            settings_window.set_always_on_top(opposite).unwrap();
        }
        None => {}
    }

    main_window.set_always_on_top(opposite).unwrap();
    always_on_top_handle.set_selected(opposite).unwrap();
    state.lock().unwrap().always_on_top = opposite;
}

pub fn toggle_visibility(app: &AppHandle) {
    let toggle_handle = app.tray_handle().get_item(MenuItemId::Toggle.as_str());
    let window = app.get_window(WindowId::Main.as_str()).unwrap();

    if window.is_visible().unwrap() {
        window.hide().unwrap();
        toggle_handle.set_title(MenuItemTitle::Show.as_str()).unwrap();
    } else {
        window.show().unwrap();
        toggle_handle.set_title(MenuItemTitle::Hide.as_str()).unwrap();
    }
}

pub fn quit(app: &AppHandle) {
    app.exit(0);
}
