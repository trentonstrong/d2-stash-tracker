// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;
use serde_json::from_str;

mod d2;

#[derive(Deserialize, Debug)]
struct CharacterDataHeader {
    name: String,
    level: u8,
    class: String,
}

#[derive(Deserialize, Debug)]
struct CharacterData {
    header: CharacterDataHeader
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn import(character_json: &str) -> String {
    let character_data: CharacterData = match from_str(character_json) {
        Ok(character_data) => character_data,
        Err(err) => return err.to_string()
    };

    println!("character: {:?}", character_data);

    return String::from("Ok");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![import])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
