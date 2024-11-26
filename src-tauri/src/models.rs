use super::schema::{project_files, projects};
use diesel::prelude::*;
use diesel::Queryable;
use diesel::Selectable;
use serde::{Deserialize, Serialize};

// #[derive(Queryable, Identifiable, AsChangeset, Serialize, Deserialize, Debug)]
// #[diesel(table_name= be::schema::platforms)]
// #[serde(rename_all = "camelCase")]
// pub struct Platform {
//     pub id: String,
//     pub name: Option<String>,
//     pub url: String,
// }

#[derive(
    Queryable, Identifiable, Insertable, Selectable, PartialEq, Serialize, Deserialize, Debug, Clone,
)]
#[diesel(table_name= projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(
    Queryable,
    Identifiable,
    Insertable,
    Selectable,
    Associations,
    PartialEq,
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
#[diesel(table_name = project_files)]
#[diesel(belongs_to(Project))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct ProjectFile {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub path: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
