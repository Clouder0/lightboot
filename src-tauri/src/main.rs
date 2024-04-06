// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::ser::{Serialize, SerializeStruct};

use std::thread;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

struct MenuEntry {
    name: String,
    start_cmd: String
}

impl Serialize for MenuEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct("MenuEntry", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("start_cmd", &self.start_cmd)?;
        state.end()
    }
}

fn menu_query(menu: i32, idx: i32) -> Vec<MenuEntry> {
    let mut V = vec![MenuEntry{name: "test".to_string(), start_cmd: "test".to_string()}];
    V.push(MenuEntry{name: idx.to_string(), start_cmd: idx.to_string()});
    V.push(MenuEntry{name: menu.to_string(), start_cmd: menu.to_string()});
    V
}

#[tauri::command]
fn upmenuquery(idx: i32) -> Vec<MenuEntry> {
    menu_query(0, idx)    
}

#[tauri::command]
fn downmenuquery(idx: i32) -> Vec<MenuEntry> {
    menu_query(1, idx)    
}

mod listener;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            thread::spawn(|| {
                let mut listener = listener::Listener::new();
                listener.init_app_handler(handle);
                listener.listen();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, upmenuquery, downmenuquery])
        .on_window_event(|_| {
            /* match event.event() {
                tauri::WindowEvent::Focused(focused) => {
                    if !focused {
                        event.window().hide().unwrap();
                    }
                }
                _ => {}
            }
            too easy to trigger, so comment it out
            */
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
