mod ffmpeg;
mod commands;

use commands::{AppState, initialize_ffmpeg, get_file_info, start_conversion, get_conversion_status, get_supported_output_formats, cancel_conversion, convert_image_to_webp, get_file_stats, get_image_dimensions};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .manage(AppState::default())
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
    .plugin(tauri_plugin_os::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_notification::init())
    .invoke_handler(tauri::generate_handler![
      initialize_ffmpeg,
      get_file_info,
      start_conversion,
      get_conversion_status,
      get_supported_output_formats,
      cancel_conversion,
      convert_image_to_webp,
      get_file_stats,
      get_image_dimensions
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
