// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn select(selection: &str) -> String {
    format!("You've selected: {}", selection)
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![select])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}