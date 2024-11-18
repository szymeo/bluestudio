// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenvy::dotenv;

mod be;
mod bff;

fn main() {
    dotenv().ok(); // Load environment variables from .env file if available

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            bff::commands::files::list_files,
            bff::commands::parse_video::parse_video,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
