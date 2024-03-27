use crate::enums::{MenuItemId, MenuItemTitle};
use tauri::{CustomMenuItem, SystemTrayMenu};

pub fn create_tray_menu() -> SystemTrayMenu {
    SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(
            MenuItemId::Settings.as_str(),
            MenuItemTitle::Settings.as_str(),
        ))
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new(
            MenuItemId::Toggle.as_str(),
            MenuItemTitle::Hide.as_str(),
        )) // Change based on initial state
        .add_item(CustomMenuItem::new(
            MenuItemId::Quit.as_str(),
            MenuItemTitle::Quit.as_str(),
        ))
}
