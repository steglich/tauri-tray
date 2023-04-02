// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, SystemTray, SystemTrayEvent, Window, AppHandle, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let tray_menu = tray_menu_fn();
    let system_tray = SystemTray::new()
        .with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
              match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide_or_show" => {
                    let window = app.get_window("main").unwrap();
                    set_hide_or_show(window, &app, &id);
                }
                _ => {}
              }
            }
            _ => {}
          })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn tray_menu_fn() -> SystemTrayMenu {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide_or_show = CustomMenuItem::new("hide_or_show".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide_or_show);
    return tray_menu;
}

fn set_hide_or_show(window: Window, app: &AppHandle, id: &String) {
    let item_handle = app.tray_handle().get_item(&id);
    let is_visible = match window.get_window("main") {
        Some(win) => win.is_visible(),
        None => Ok(false)
    };
    match is_visible {
        Ok(true) => {
            window.hide().unwrap();
            item_handle.set_title("Show").unwrap();
        },
        Ok(false) => {
            window.show().unwrap();
            item_handle.set_title("Hide").unwrap();
        },
        _ => {}
    };
}