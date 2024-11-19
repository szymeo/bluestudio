// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod be;
mod bff;
pub mod models;
pub mod schema;

use be::db;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use dotenvy::dotenv;
use std::sync::Arc;
use tauri::Manager;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Clone)]
struct AppState {
    pool: Arc<DbPool>,
}

fn main() {
    dotenv().ok(); // Load environment variables from .env file if available

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir")
                .to_str()
                .expect("failed to convert path to string")
                .to_string();

            let db_path = db::init(&app_data_dir).expect("Failed to initialize database");

            let pool = db::create_pool(&db_path).expect("Failed to create database pool");

            // let menu = menu::create_menu(&app.handle())?;
            // app.set_menu(menu)?;

            // Get initial base_currency from settings
            // let mut conn = pool.get().expect("Failed to get database connection");
            // let settings_service = settings::SettingsService::new();

            // Get instance_id from settings
            // let settings = settings_service.get_settings(&mut conn)?;
            // let instance_id = settings.instance_id.clone();

            // Initialize state
            let state = AppState { pool };
            app.manage(state.clone());

            let handle = app.handle().clone();
            // Check for updates on startup
            // spawn(async move { check_for_update(handle, &instance_id, false).await });

            // Sync quotes on startup
            // spawn_quote_sync(app.handle().clone(), state);

            // Set up the menu event handler
            // let instance_id_clone = settings.instance_id.clone();
            // app.on_menu_event(move |app, event| {
            //     menu::handle_menu_event(app, &instance_id_clone, event.id().as_ref());
            // });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            bff::commands::files::list_files,
            bff::commands::parse_video::parse_video,
            bff::commands::projects::create_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
