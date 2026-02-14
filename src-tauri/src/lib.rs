use tauri::Manager;

mod image_converter;

// To be called once the app has initialized
#[tauri::command]
fn show_window(app: tauri::AppHandle) {
    app.get_webview_window("main")
        .expect("Failed to get window 'main'")
        .show()
        .expect("Failed to show window 'main'");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            show_window,
            image_converter::convert_images
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
