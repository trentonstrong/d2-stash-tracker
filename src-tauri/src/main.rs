// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::trace;
use serde_json::from_str;
use d2::model::CharacterData;

mod d2;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn import(character_json: &str) -> String {
    trace!("importing character: {}", character_json);
    let character_data: CharacterData = match from_str(character_json) {
        Ok(character_data) => character_data,
        Err(err) => return err.to_string()
    };

    trace!("character: {:?}", character_data);

    return String::from("Ok");
}

fn main() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "debug");
    env_logger::init_from_env(env);
    log::info!("Starting up D2Rust Stash Manager");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![import])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
