// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Window, Wry, SystemTray, SystemTrayMenu, CustomMenuItem, SystemTrayEvent, AppHandle};

#[tauri::command]
async fn toggle_window(window: Window<Wry>) -> Result<bool, String> {
    match window.is_visible() {
        Ok(is_visible) => {
            if is_visible {
                window.hide().unwrap();
            } else {
                window.show().unwrap();
            }
        }
        Err(e) => {
            return Err(format!("Failed to get window visibility: {}", e));
        }
    }
    Ok(true)
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  args: Vec<String>,
  cwd: String,
}

fn show_window(app: &AppHandle<Wry>) {
    match app.get_window("main") {
        Some(window) => {
            if !window.is_visible().unwrap_or(false) {
                window.show().unwrap();
            }
            window
                .set_focus().unwrap();
        }
        _ => {
            //Error
        }
    };
}


fn create_tray() -> SystemTray {
    // Define menu items for the system tray
    let show = CustomMenuItem::new("show".to_string(), "Show Window");

    // Create the tray menu with the defined menu items
    let tray_menu = SystemTrayMenu::new().add_item(show);

    SystemTray::new().with_menu(tray_menu)
}
pub fn tray_event_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::DoubleClick { .. } => show_window(app),
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "show" => show_window(app),
                _ => {}
            }
        }
        _ => {}
    }
}


fn main() {
    tauri::Builder::default()
    .system_tray(create_tray())
    .on_system_tray_event(tray_event_handler)
        .invoke_handler(tauri::generate_handler![toggle_window])
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            app.emit_all("single-instance", Payload { args: argv, cwd }).unwrap();
        }))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
