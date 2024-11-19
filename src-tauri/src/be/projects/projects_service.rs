use crate::be::projects::projects_repository::ProjectsRepository;
use crate::models::Project;
use diesel::Connection;
use diesel::SqliteConnection;
use log::debug;

pub struct ProjectsService {
    projects_repo: ProjectsRepository,
}

impl ProjectsService {
    pub fn new() -> Self {
        ProjectsService {
            projects_repo: ProjectsRepository::new(),
        }
    }

    pub fn get_projects(
        &self,
        conn: &mut SqliteConnection,
    ) -> Result<Vec<Project>, diesel::result::Error> {
        self.projects_repo.load_projects(conn)
    }

    // pub fn get_active_accounts(
    //     &self,
    //     conn: &mut SqliteConnection,
    // ) -> Result<Vec<Account>, diesel::result::Error> {
    //     self.account_repo.load_active_accounts(conn)
    // }

    // pub fn get_account_by_id(
    //     &self,
    //     conn: &mut SqliteConnection,
    //     account_id: &str,
    // ) -> Result<Account, diesel::result::Error> {
    //     self.account_repo.load_account_by_id(conn, account_id)
    // }

    pub async fn create_project(
        &self,
        conn: &mut SqliteConnection,
        name: String,
    ) -> Result<Project, Box<dyn std::error::Error>> {
        debug!("Creating project..., name: {}", name);

        conn.transaction(|conn| {
            self.projects_repo
                .insert_project(conn, name)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        })
    }

    // pub fn update_account(
    //     &self,
    //     conn: &mut SqliteConnection,
    //     updated_account_data: AccountUpdate,
    // ) -> Result<Account, diesel::result::Error> {
    //     self.account_repo.update_account(conn, updated_account_data)
    // }

    // pub fn delete_account(
    //     &self,
    //     conn: &mut SqliteConnection,
    //     account_id_to_delete: String,
    // ) -> Result<usize, diesel::result::Error> {
    //     self.account_repo.delete_account(conn, account_id_to_delete)
    // }

    // pub fn get_accounts_by_ids(
    //     &self,
    //     conn: &mut SqliteConnection,
    //     account_ids: &[String],
    // ) -> Result<Vec<Account>, diesel::result::Error> {
    //     self.account_repo.load_accounts_by_ids(conn, account_ids)
    // }
}
