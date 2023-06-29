// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
use tauri_plugin_log::LogTarget;
use window_vibrancy::{apply_acrylic};
// use window_shadows::set_shadow;

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
            let window = app.get_window("main").unwrap();
            let _ = window.set_decorations(true); // override default decorations
            // let _ = set_shadow(&window, true); Don't use unwrap() here as it will panic on Linux.
            apply_acrylic(&window, Some((1, 1, 1, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windowsp");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![command::greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
