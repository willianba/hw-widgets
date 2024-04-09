// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
#![warn(clippy::all, rust_2018_idioms)]

mod enums;
mod sensors;
mod tray;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use enums::{MenuItemId, MenuItemTitle, WindowId, WindowTitle};
use sensors::{SensorData, Sensors};
use tauri::{Manager, State, SystemTray, SystemTrayEvent, WindowBuilder, WindowEvent, WindowUrl};
use tokio::time;
use tray::{
    events::{quit, toggle_always_on_top, toggle_visibility},
    menu::create_tray_menu,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn stats(state: State<'_, Arc<Mutex<AppState>>>) -> Vec<SensorData> {
    state.lock().unwrap().sensor_data.clone()
}

#[derive(Clone, Default)]
pub struct AppState {
    pub always_on_top: bool,
    pub sensor_data: Vec<SensorData>,
}

#[tokio::main]
async fn main() {
    let mut sensors_manager = Sensors::new();
    let state = Arc::new(Mutex::new(AppState::default()));

    tokio::spawn({
        let state = state.clone();
        async move {
            let mut interval = time::interval(Duration::from_secs(2));
            loop {
                interval.tick().await;
                state.lock().unwrap().sensor_data = sensors_manager.get_data();
            }
        }
    });

    tauri::Builder::default()
        .manage(state.clone())
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
        .system_tray(SystemTray::new().with_menu(create_tray_menu()))
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::LeftClick { position: _, size: _, .. } => {
                let toggle_handle = app.tray_handle().get_item(MenuItemId::Toggle.as_str());
                let window = app.get_window(WindowId::Main.as_str()).unwrap();

                window.show().unwrap();
                toggle_handle.set_title(MenuItemTitle::Hide.as_str()).unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "always_on_top" => toggle_always_on_top(app, state.clone()),
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
