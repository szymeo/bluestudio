use crate::be::projects::projects_service::ProjectsService;
use crate::models::Project;
use crate::AppState;
use log::debug;
use tauri::State;

#[tauri::command]
pub async fn create_project(name: String, state: State<'_, AppState>) -> Result<Project, String> {
    debug!("Adding new project...");
    print!("LULULUULULULULUL;");
    let mut conn = state
        .pool
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e))?;

    let service = ProjectsService::new();
    service
        .create_project(&mut conn, name)
        .await
        .map_err(|e| format!("Failed to add new project: {}", e))
}
