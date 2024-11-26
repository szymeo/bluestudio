use crate::be::project_files::project_files_repository::{
    ProjectFileQuery, ProjectFilesRepository,
};
use crate::models::ProjectFile;
use diesel::Connection;
use diesel::SqliteConnection;
use log::debug;

pub struct ProjectFilesService {
    project_files_repo: ProjectFilesRepository,
}

impl ProjectFilesService {
    pub fn new() -> Self {
        ProjectFilesService {
            project_files_repo: ProjectFilesRepository::new(),
        }
    }

    pub fn get_project_files(
        &self,
        conn: &mut SqliteConnection,
        project_id: &str,
    ) -> Result<Vec<ProjectFileQuery>, diesel::result::Error> {
        self.project_files_repo.load_project_files(conn, project_id)
    }

    pub async fn create_project_file(
        &self,
        conn: &mut SqliteConnection,
        project_id: String,
        path: String,
    ) -> Result<ProjectFile, Box<dyn std::error::Error>> {
        debug!(
            "Creating project file..., project_id: {}, path: {}",
            project_id, path
        );

        conn.transaction(|conn| {
            self.project_files_repo
                .insert_project_file(conn, project_id, path)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        })
    }
}
