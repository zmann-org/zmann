// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use actix_web::{HttpServer, App};
use log::info;
use tauri_plugin_log::{LogTarget};

mod api;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}



fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().targets([
            LogTarget::LogDir,
            LogTarget::Stdout,
            LogTarget::Webview,
        ]).build())
        .setup(|app| {
            let address = ("127.0.0.1", 8080);
            tauri::async_runtime::spawn(
                HttpServer::new(|| App::new().service(api::endpoints::hello))
                    .bind(address)?
                    .run(),
            );
            info!("web server started at http://{}:{}", address.0, address.1);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
