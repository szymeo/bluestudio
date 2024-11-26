use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
pub struct FSFileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
}

pub fn resolve_for(file_path: &String) -> Result<FSFileInfo, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    let metadata = fs::metadata(path)?;
    if metadata.is_file() {
        let name = path
            .file_name()
            .ok_or("Failed to get file name")?
            .to_string_lossy()
            .into_owned();
        let path = path.to_string_lossy().into_owned();
        let size = metadata.len();
        Ok(FSFileInfo { name, path, size })
    } else {
        Err("Path is not a file".into())
    }
}
