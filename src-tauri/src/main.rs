// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use actix_web::{HttpServer, App};

mod api;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}



fn main() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::spawn(
                HttpServer::new(|| App::new().service(api::endpoints::hello))
                    .bind(("127.0.0.1", 8080))?
                    .run(),
            );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
