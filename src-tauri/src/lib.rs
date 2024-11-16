use serde::Serialize;
use std::fs;
use std::path::PathBuf;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize)]
struct FileInfo {
    name: String,
    path: String,
    is_dir: bool,
}

#[tauri::command]
fn list_files(dir: String) -> Result<Vec<FileInfo>, String> {
    let path = PathBuf::from(dir);
    let mut files = Vec::new();

    if path.is_dir() {
        for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            let name = entry
                .file_name()
                .into_string()
                .map_err(|e| e.to_string_lossy().into_owned())?;
            let is_dir = path.is_dir();

            files.push(FileInfo {
                name,
                path: path.to_string_lossy().into_owned(),
                is_dir,
            });
        }
    }

    Ok(files)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, list_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
