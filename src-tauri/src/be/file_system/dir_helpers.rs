use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct FSDirEntry {
    name: String,
    path: String,
    is_dir: bool,
}

pub fn list_dir(dir: String) -> Result<Vec<FSDirEntry>, String> {
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

            files.push(FSDirEntry {
                name,
                path: path.to_string_lossy().into_owned(),
                is_dir,
            });
        }
    }

    Ok(files)
}
