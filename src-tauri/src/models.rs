use super::schema::projects;
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
