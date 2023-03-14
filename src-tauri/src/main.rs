// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use active_win_pos_rs::get_active_window;
use std::collections::HashMap;
use std::sync::mpsc;
use std::{thread, time};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_window_name() -> String {
    match get_active_window() {
        Ok(active_window) => {
            return active_window.process_name;
        }
        Err(()) => {
            return "".into();
        }
    }
}

fn main() {
    let two_secs = time::Duration::from_millis(3000);
    let (tracker_tx, tracker_rx) = mpsc::channel();
    let mut activities = HashMap::new();

    let monitor_handle =
        thread::spawn(move || twospan::app_tracker::monitor_loop("", two_secs, tracker_tx));
    let tracker_handle =
        thread::spawn(move || twospan::app_tracker::activity_loop(tracker_rx, &mut activities));
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_window_name])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
