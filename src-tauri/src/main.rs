// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod enums;
mod sensors;
mod state;
mod tray;

use std::time::Duration;

use enums::{MenuItemId, MenuItemTitle, WindowId, WindowTitle};
use sensors::SensorData;
use state::SensorState;
use tauri::{Manager, SystemTray, SystemTrayEvent, WindowBuilder, WindowEvent, WindowUrl};
use tokio::time;
use tray::{
    events::{quit, toggle_visibility},
    menu::create_tray_menu,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn stats() -> Vec<SensorData> {
    sensors::get_sensor_data()
}

#[tokio::main]
async fn main() {
    let sensor_state = SensorState::default();
    let sensor_data_for_loop = sensor_state.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(2));
        loop {
            interval.tick().await;
            sensor_data_for_loop.update().await;
        }
    });

    let tray = SystemTray::new().with_menu(create_tray_menu());

    tauri::Builder::default()
        .manage(sensor_state)
        .on_window_event(|event| match event.event() {
            WindowEvent::CloseRequested { api, .. } => match event.window().label() {
                "main" => {
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
            },
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
                "settings" => {
                    WindowBuilder::new(
                        app,
                        WindowId::Settings.as_str(),
                        WindowUrl::App("settings".into()),
                    )
                    .title(WindowTitle::Settings.as_str())
                    .center()
                    .build()
                    .expect("Failed to create settings window");
                }
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
