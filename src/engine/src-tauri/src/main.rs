#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use window_vibrancy::{apply_blur, apply_vibrancy, apply_acrylic,apply_mica, NSVisualEffectMaterial};

// apply_acrylic(&window, Some((18, 18, 18, 125))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .setup(|app| {
        use tauri::Manager;
        use window_shadows::set_shadow;

        let window = app.get_window("main").unwrap();
        #[cfg(target_os = "windows")]
        // apply_mica(&window).expect("Unsupported platform! 'apply_blur' is only supported on Windows");
      apply_acrylic(&window, Some((18, 18, 18, 125))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");


        let _ = set_shadow(&window, true); // Don't use unwrap() here as it will panic on Linux.
        Ok(())
    })
    .run(context)
    .expect("error while running tauri application");
}