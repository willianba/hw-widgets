// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod enums;
mod sensors;
mod tray;

use enums::{MenuItemId, MenuItemTitle};
use sensors::SensorData;
use tauri::{Manager, SystemTray, SystemTrayEvent, WindowEvent};
use tray::{
    events::{quit, toggle_visibility},
    menu::create_tray_menu,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn stats() -> Vec<SensorData> {
    let data = sensors::get_sensor_data().expect("Failed to get sensor data");
    let parsed = sensors::parse_sensor_data(data);
    parsed
}

fn main() {
    let tray = SystemTray::new().with_menu(create_tray_menu());

    tauri::Builder::default()
        .on_window_event(|event| match event.event() {
            WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                event.window().hide().unwrap();
                event
                    .window()
                    .app_handle()
                    .tray_handle()
                    .get_item(MenuItemId::Toggle.as_str())
                    .set_title(MenuItemTitle::Show.as_str())
                    .unwrap();
            }
            _ => {}
        })
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                toggle_visibility(app);
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "toggle" => {
                    toggle_visibility(app);
                }
                "quit" => {
                    quit(app);
                }
                _ => {}
            },
            _ => (),
        })
        .invoke_handler(tauri::generate_handler![stats])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
