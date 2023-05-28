// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use log::trace;
use d2_stash_tracker::{get_connection_pool, import_character, run_pending_migrations, ConnectionPool};
use diesel::prelude::*;


#[tauri::command]
fn import(pool: tauri::State<ConnectionPool>, character_json: &str) -> Result<String, String> {
    trace!("importing character: {}", character_json);
    let conn = &mut pool.get().unwrap();
    let result = import_character(conn, character_json).map_err(|err| err.to_string())?;
    trace!("result: {:?}", result);
    let result_msg = format!("{} character {} successfully.", result.status, result.character.name);

    return Ok(result_msg);
}

fn main() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "debug");
    env_logger::init_from_env(env);
    log::info!("Starting up D2Rust Stash Manager");

    let pool = get_connection_pool(None);
    let conn: &mut SqliteConnection = &mut pool.get().unwrap();
    run_pending_migrations(conn).unwrap();

    tauri::Builder::default()
        .manage(pool)
        .invoke_handler(tauri::generate_handler![import])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
