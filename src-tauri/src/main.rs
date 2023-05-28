// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use actix_web::{App, HttpServer};
use log::{info, warn};
use tauri::Manager;
use tauri_plugin_log::LogTarget;

#[cfg(target_os = "windows")]
use window_vibrancy::apply_acrylic;

mod api;
mod command;

fn main() {
    tauri::Builder::default()
        .on_window_event(|e| {
            if let tauri::WindowEvent::Resized(_) = e.event() {
                std::thread::sleep(std::time::Duration::from_nanos(1));
            }
        })
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .setup(|app| {
            let address = ("127.0.0.1", 55123);
            tauri::async_runtime::spawn(
                HttpServer::new(|| App::new().service(api::endpoints::hello))
                    .bind(address)?
                    .run(),
            );
            warn!("web server started at http://{}:{}", address.0, address.1);

            let window = app.get_window("main").unwrap();
            apply_acrylic(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windowsp");

            let _ = window.set_decorations(true);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![command::greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
