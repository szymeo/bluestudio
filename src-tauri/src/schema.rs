// @generated automatically by Diesel CLI.

diesel::table! {
    project_files (id) {
        id -> Text,
        project_id -> Text,
        name -> Text,
        path -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    projects (id) {
        id -> Text,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(project_files -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(project_files, projects,);
