use crate::be::project_files::project_files_repository::ProjectFileQuery;
use crate::be::project_files::project_files_service::ProjectFilesService;
use crate::models::ProjectFile;
use crate::AppState;
use log::debug;
use tauri::State;

#[tauri::command]
pub fn get_project_files(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ProjectFileQuery>, String> {
    debug!("Getting project files...");
    let mut conn = state
        .pool
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e))?;

    let service = ProjectFilesService::new();
    service
        .get_project_files(&mut conn, &project_id)
        .map_err(|e| format!("Failed to get project files: {}", e))
}

#[tauri::command]
pub async fn create_project_file(
    project_id: String,
    path: String,
    state: State<'_, AppState>,
) -> Result<ProjectFile, String> {
    debug!("Adding new project file...");
    let mut conn = state
        .pool
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e))?;

    let service = ProjectFilesService::new();
    service
        .create_project_file(&mut conn, project_id, path)
        .await
        .map_err(|e| format!("Failed to add new project: {}", e))
}
