use crate::ffmpeg::{convert_media, ensure_ffmpeg, get_media_info, get_supported_formats, cancel_conversion as cancel_ffmpeg_conversion, MediaInfo};
use tauri::{AppHandle, State};
use std::sync::Arc;
use tokio::sync::Mutex;

// Windows-specific imports for hiding CMD windows
#[cfg(windows)]
use std::os::windows::process::CommandExt;

// Windows constant for hiding console window
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Default)]
pub struct AppState {
    pub is_converting: Arc<Mutex<bool>>,
}

#[tauri::command]
pub async fn initialize_ffmpeg() -> Result<(), String> {
    ensure_ffmpeg().await
}

#[tauri::command]
pub async fn get_file_info(file_path: String) -> Result<MediaInfo, String> {
    get_media_info(&file_path).await
}

#[tauri::command]
pub async fn start_conversion(
    input_path: String,
    output_path: String,
    format: String,
    quality: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    duration: Option<f64>,
    duration_mode: Option<String>, // New parameter
    fps: Option<f64>,
    disable_audio: Option<bool>, // Add disable audio parameter
    app_handle: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let is_converting = state.is_converting.clone();
    let mut converting = is_converting.lock().await;
    
    if *converting {
        return Err("A conversion is already in progress".to_string());
    }
    
    *converting = true;
    drop(converting);

    let quality_ref = quality.as_deref();
    let duration_mode_ref = duration_mode.as_deref();
    let result = convert_media(
        &input_path, 
        &output_path, 
        &format, 
        quality_ref, 
        width, 
        height, 
        duration, 
        duration_mode_ref, // Pass the new parameter
        fps, 
        disable_audio, // Pass the disable audio parameter
        app_handle
    ).await;
    
    let mut converting = is_converting.lock().await;
    *converting = false;
    
    result
}

#[tauri::command]
pub async fn get_conversion_status(state: State<'_, AppState>) -> Result<bool, ()> {
    let is_converting = state.is_converting.lock().await;
    Ok(*is_converting)
}

#[tauri::command]
pub fn get_supported_output_formats() -> Vec<&'static str> {
    get_supported_formats()
}

#[tauri::command]
pub async fn cancel_conversion(state: State<'_, AppState>) -> Result<(), String> {
    let mut converting = state.is_converting.lock().await;
    *converting = false;
    
    // Cancel the actual FFmpeg conversion
    cancel_ffmpeg_conversion()?;
    
    Ok(())
}

#[tauri::command]
pub async fn convert_image_to_webp(
    input_path: String,
    app_handle: AppHandle,
) -> Result<String, String> {
    crate::ffmpeg::convert_image_to_webp(&input_path, app_handle).await
}

#[tauri::command]
pub async fn get_file_stats(file_path: String) -> Result<serde_json::Value, String> {
    use std::fs;
    
    let metadata = fs::metadata(&file_path)
        .map_err(|e| format!("Failed to get file metadata: {}", e))?;
    
    let file_size = metadata.len();
    
    // Try to get image dimensions using the existing get_media_info function
    if let Ok(media_info) = crate::ffmpeg::get_media_info(&file_path).await {
        Ok(serde_json::json!({
            "size": file_size,
            "width": media_info.width,
            "height": media_info.height
        }))
    } else {
        Ok(serde_json::json!({
            "size": file_size
        }))
    }
}

#[tauri::command]
pub async fn get_image_dimensions(path: String) -> Result<(u32, u32), String> {
    println!("Getting image dimensions for: {}", path);
    let dimensions = get_image_dimensions_ffprobe(&path).await?;
    println!("Image dimensions: {}x{}", dimensions.0, dimensions.1);
    Ok(dimensions)
}

async fn get_image_dimensions_ffprobe(path: &str) -> Result<(u32, u32), String> {
    // Get the path to ffprobe executable
    let ffprobe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?
        .parent()
        .ok_or("Failed to get parent directory")?
        .join("bin")
        .join("ffprobe-x86_64-pc-windows-msvc");

    // Run ffprobe command to get video/image info as JSON
    let mut cmd = std::process::Command::new(&ffprobe_path);
    cmd.args([
        "-v", "quiet",
        "-print_format", "json",
        "-show_streams",
        path
    ])
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::piped());

    // Hide CMD window on Windows
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let output = cmd.output()
        .map_err(|e| format!("Failed to execute ffprobe: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("ffprobe failed: {}", stderr));
    }

    // Parse JSON output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse ffprobe output: {}", e))?;

    // Extract width and height from the first video/image stream
    let streams = json["streams"].as_array()
        .ok_or("No streams found in ffprobe output")?;

    for stream in streams {
        if let (Some(width), Some(height)) = (
            stream["width"].as_u64(),
            stream["height"].as_u64()
        ) {
            return Ok((width as u32, height as u32));
        }
    }

    Err("No video/image stream with dimensions found".to_string())
}
