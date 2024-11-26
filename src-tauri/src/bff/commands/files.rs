use crate::be::file_system::dir_helpers;

#[tauri::command]
pub fn list_files(dir: String) -> Result<Vec<dir_helpers::FSDirEntry>, String> {
    dir_helpers::list_dir(dir)
}
