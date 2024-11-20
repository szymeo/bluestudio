use crate::models::Project;
use crate::schema::projects;
use crate::schema::projects::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

pub struct ProjectsRepository;

impl ProjectsRepository {
    pub fn new() -> Self {
        ProjectsRepository
    }

    // pub fn load_account_by_id(
    //     &self,
    //     conn: &mut SqliteConnection,
    //     account_id: &str,
    // ) -> Result<Account, diesel::result::Error> {
    //     accounts.find(account_id).first::<Account>(conn)
    // }

    pub fn load_projects(
        &self,
        conn: &mut SqliteConnection,
    ) -> Result<Vec<Project>, diesel::result::Error> {
        projects.order(created_at.desc()).load::<Project>(conn)
    }

    pub fn insert_project(
        &self,
        conn: &mut SqliteConnection,
        project_name: String,
    ) -> Result<Project, diesel::result::Error> {
        let new_project = Project {
            id: Uuid::new_v4().to_string(),
            name: project_name,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        diesel::insert_into(projects::table)
            .values(&new_project)
            .returning(projects::all_columns)
            .get_result(conn)
    }

    // pub fn update_account(
    //     &self,
    //     conn: &mut SqliteConnection,
    //     account_update: AccountUpdate, // Assuming AccountUpdate is the struct for updating account data
    // ) -> Result<Account, diesel::result::Error> {
    //     use crate::schema::accounts::dsl::*;

    //     // Clone the id before unwrapping it
    //     let account_id = account_update.id.clone().unwrap();

    //     diesel::update(accounts.find(account_id))
    //         .set(&account_update)
    //         .execute(conn)?;

    //     accounts
    //         .filter(id.eq(account_update.id.unwrap()))
    //         .first(conn)
    // }

    // pub fn delete_account(
    //     &self,
    //     conn: &mut SqliteConnection,
    //     account_id: String, // ID of the account to delete
    // ) -> Result<usize, diesel::result::Error> {
    //     use crate::schema::accounts::dsl::*;

    //     diesel::delete(accounts.filter(id.eq(account_id))).execute(conn)
    // }

    // pub fn load_accounts_by_ids(
    //     &self,
    //     conn: &mut SqliteConnection,
    //     account_ids: &[String],
    // ) -> Result<Vec<Account>, diesel::result::Error> {
    //     accounts
    //         .filter(id.eq_any(account_ids))
    //         .filter(is_active.eq(true))
    //         .order(created_at.desc())
    //         .load::<Account>(conn)
    // }
}
