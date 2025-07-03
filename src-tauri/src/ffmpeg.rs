use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};
use std::io::{BufRead, BufReader};
use std::thread;

// Windows-specific imports for hiding CMD windows
#[cfg(windows)]
use std::os::windows::process::CommandExt;

// Windows constant for hiding console window
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionProgress {
    pub frame: Option<u64>,
    pub fps: Option<f64>,
    pub bitrate: Option<String>,
    pub total_size: Option<u64>,
    pub out_time_us: Option<u64>,
    pub speed: Option<f64>,
    pub progress: Option<String>,
    pub percentage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaInfo {
    pub duration: Option<f64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub bitrate: Option<String>,
    pub fps: Option<f64>,
    pub file_size: Option<u64>,
}

// Global cancellation flag
pub static CONVERSION_CANCELLED: Mutex<bool> = Mutex::new(false);

pub fn get_ffmpeg_path() -> Result<PathBuf, String> {
    // Get the path to the bundled FFmpeg executable
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("Failed to get current exe path: {}", e))?
        .parent()
        .ok_or("Failed to get exe parent directory")?
        .to_path_buf();

    let ffmpeg_path = exe_dir.join("ffmpeg-x86_64-pc-windows-msvc");
    
    if ffmpeg_path.exists() {
        Ok(ffmpeg_path)
    } else {
        // Fallback to system FFmpeg
        Ok(PathBuf::from("ffmpeg"))
    }
}

pub async fn ensure_ffmpeg() -> Result<(), String> {
    let ffmpeg_path = get_ffmpeg_path()?;
    
    // Test if FFmpeg is working
    let mut cmd = Command::new(&ffmpeg_path);
    cmd.arg("-version").stdout(Stdio::piped()).stderr(Stdio::piped());
    
    // Hide CMD window on Windows
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    
    let output = cmd.output()
        .map_err(|e| format!("Failed to execute FFmpeg: {}. Make sure FFmpeg is installed or bundled.", e))?;

    if output.status.success() {
        let version_info = String::from_utf8_lossy(&output.stdout);
        log::info!("FFmpeg found and working: {}", version_info.lines().next().unwrap_or("Unknown version"));
        Ok(())
    } else {
        Err("FFmpeg executable found but not working properly".to_string())
    }
}

pub async fn get_media_info(input_path: &str) -> Result<MediaInfo, String> {
    let ffmpeg_path = get_ffmpeg_path()?;
    
    // Use FFmpeg to get media information
    let mut cmd = Command::new(&ffmpeg_path);
    cmd.args([
        "-i", input_path,
        "-f", "null", 
        "-"
    ])
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());
    
    // Hide CMD window on Windows
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    
    let output = cmd.output()
        .map_err(|e| format!("Failed to execute FFmpeg: {}", e))?;

    // FFmpeg outputs media info to stderr
    let error_output = String::from_utf8_lossy(&output.stderr);
    
    let mut media_info = MediaInfo {
        duration: None,
        width: None,
        height: None,
        video_codec: None,
        audio_codec: None,
        bitrate: None,
        fps: None,
        file_size: None,
    };

    // Parse the FFmpeg output
    for line in error_output.lines() {
        // Parse duration
        if line.contains("Duration:") {
            if let Some(duration_str) = extract_duration_from_line(line) {
                media_info.duration = parse_duration(&duration_str);
            }
        }
        
        // Parse video stream info
        if line.contains("Stream") && line.contains("Video:") {
            if let Some((codec, width, height, fps)) = extract_video_info_from_line(line) {
                media_info.video_codec = Some(codec);
                media_info.width = width;
                media_info.height = height;
                media_info.fps = fps;
            }
        }
        
        // Parse audio stream info
        if line.contains("Stream") && line.contains("Audio:") {
            if let Some(codec) = extract_audio_codec_from_line(line) {
                media_info.audio_codec = Some(codec);
            }
        }
        
        // Parse bitrate
        if line.contains("bitrate:") {
            if let Some(bitrate) = extract_bitrate_from_line(line) {
                media_info.bitrate = Some(bitrate);
            }
        }
    }

    Ok(media_info)
}

fn extract_duration_from_line(line: &str) -> Option<String> {
    if let Some(start) = line.find("Duration: ") {
        let start = start + 10; // "Duration: ".len()
        if let Some(end) = line[start..].find(',') {
            return Some(line[start..start + end].to_string());
        }
    }
    None
}

fn parse_duration(duration_str: &str) -> Option<f64> {
    let parts: Vec<&str> = duration_str.split(':').collect();
    if parts.len() == 3 {
        if let (Ok(hours), Ok(minutes), Ok(seconds)) = (
            parts[0].parse::<f64>(),
            parts[1].parse::<f64>(),
            parts[2].parse::<f64>(),
        ) {
            return Some(hours * 3600.0 + minutes * 60.0 + seconds);
        }
    }
    None
}

fn extract_video_info_from_line(line: &str) -> Option<(String, Option<u32>, Option<u32>, Option<f64>)> {
    let mut codec = None;
    let mut width = None;
    let mut height = None;
    let mut fps = None;
    
    // Extract codec
    if let Some(video_pos) = line.find("Video: ") {
        let after_video = &line[video_pos + 7..];
        if let Some(codec_end) = after_video.find(' ') {
            codec = Some(after_video[..codec_end].to_string());
        }
    }
    
    // Extract resolution
    let parts: Vec<&str> = line.split(&[',', ' ']).collect();
    for part in parts {
        if part.contains('x') && !part.contains('@') && !part.contains('.') {
            if let Some(x_pos) = part.find('x') {
                let width_str = &part[..x_pos];
                let height_str = &part[x_pos + 1..];
                
                if width_str.chars().all(|c| c.is_ascii_digit()) && 
                   height_str.chars().all(|c| c.is_ascii_digit()) {
                    width = width_str.parse().ok();
                    height = height_str.parse().ok();
                    break;
                }
            }
        }
    }
    
    // Extract FPS
    if let Some(fps_pos) = line.find(" fps") {
        let before_fps = &line[..fps_pos];
        if let Some(fps_start) = before_fps.rfind(' ') {
            if let Ok(fps_val) = before_fps[fps_start + 1..].parse::<f64>() {
                fps = Some(fps_val);
            }
        }
    }
    
    if let Some(codec) = codec {
        Some((codec, width, height, fps))
    } else {
        None
    }
}

fn extract_audio_codec_from_line(line: &str) -> Option<String> {
    if let Some(audio_pos) = line.find("Audio: ") {
        let after_audio = &line[audio_pos + 7..];
        if let Some(codec_end) = after_audio.find(' ') {
            return Some(after_audio[..codec_end].to_string());
        }
    }
    None
}

fn extract_bitrate_from_line(line: &str) -> Option<String> {
    if let Some(bitrate_pos) = line.find("bitrate: ") {
        let after_bitrate = &line[bitrate_pos + 9..];
        if let Some(bitrate_end) = after_bitrate.find(' ') {
            return Some(after_bitrate[..bitrate_end].to_string());
        }
    }
    None
}

pub async fn convert_media(
    input_path: &str,
    output_path: &str,
    format: &str,
    quality: Option<&str>,
    width: Option<u32>,
    height: Option<u32>,
    duration: Option<f64>,
    duration_mode: Option<&str>,
    fps: Option<f64>,
    disable_audio: Option<bool>,
    app_handle: AppHandle,
) -> Result<(), String> {
    // Reset cancellation flag
    {
        let mut cancelled = CONVERSION_CANCELLED.lock().unwrap();
        *cancelled = false;
    }

    let ffmpeg_path = get_ffmpeg_path()?;

    // Helper: image extensions
    fn is_image_format(fmt: &str) -> bool {
        matches!(fmt.to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "webp" | "bmp" | "gif" | "tiff" | "ico")
    }

    let input_ext = std::path::Path::new(input_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    let output_ext = std::path::Path::new(output_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let is_image = is_image_format(&input_ext) && is_image_format(&output_ext);

    if is_image {
        return convert_image_simple(input_path, output_path, quality, width, height).await;
    }

    let mut args = vec!["-i".to_string(), input_path.to_string()];

    // Add overwrite flag early
    args.push("-y".to_string());

    // Handle duration parameter based on mode
    if let Some(duration_secs) = duration {
        match duration_mode.unwrap_or("trim") {
            "trim" => {
                args.extend(["-t".to_string(), duration_secs.to_string()]);
            }
            "compress" => {
                if let Ok(media_info) = get_media_info(input_path).await {
                    if let Some(original_duration) = media_info.duration {
                        let speed_factor = original_duration / duration_secs;
                        let video_filter = if let (Some(w), Some(h)) = (width, height) {
                            format!(r"scale={}:{},setpts=PTS/{}", w, h, speed_factor)
                        } else {
                            format!("setpts=PTS/{}", speed_factor)
                        };
                        args.extend(["-filter:v".to_string(), video_filter]);
                        if speed_factor >= 0.5 && speed_factor <= 4.0 {
                            args.extend(["-filter:a".to_string(), format!("atempo={}", speed_factor)]);
                        } else {
                            args.extend(["-an".to_string()]);
                        }
                    }
                }
            }
            _ => {
                args.extend(["-t".to_string(), duration_secs.to_string()]);
            }
        }
    } else if let (Some(w), Some(h)) = (width, height) {
        args.extend(["-vf".to_string(), format!("scale={}:{}", w, h)]);
    }

    if let Some(target_fps) = fps {
        args.extend(["-r".to_string(), target_fps.to_string()]);
    }

    if disable_audio.unwrap_or(false) {
        args.extend(["-an".to_string()]);
    }

    let crf_value = match quality {
        Some("low") => Some("28"),
        Some("medium") => Some("23"),
        Some("high") => Some("18"),
        Some(q) if q.parse::<u8>().is_ok() => Some(q),
        _ => Some("23"),
    };

    match format {
        "mp4" => {
            args.extend(["-c:v".to_string(), "libx264".to_string()]);
            if let Some(crf) = crf_value {
                args.extend(["-crf".to_string(), crf.to_string()]);
            }
            args.extend(["-preset".to_string(), "medium".to_string()]);
        }
        "webm" => {
            args.extend(["-c:v".to_string(), "libvpx-vp9".to_string()]);
            if let Some(crf) = crf_value {
                args.extend(["-crf".to_string(), crf.to_string()]);
            }
            args.extend(["-b:v".to_string(), "0".to_string()]); // For constant quality mode
        }
        "avi" => {
            args.extend(["-c:v".to_string(), "libx264".to_string()]);
            if let Some(crf) = crf_value {
                args.extend(["-crf".to_string(), crf.to_string()]);
            }
        }
        "mov" | "mkv" => {
            args.extend(["-c:v".to_string(), "libx264".to_string()]);
            if let Some(crf) = crf_value {
                args.extend(["-crf".to_string(), crf.to_string()]);
            }
        }
        _ => {
            args.extend(["-c:v".to_string(), "libx264".to_string()]);
            if let Some(crf) = crf_value {
                args.extend(["-crf".to_string(), crf.to_string()]);
            }
        }
    }

    // Add progress reporting
    args.extend(["-progress".to_string(), "pipe:1".to_string()]);
    
    // Output file
    args.push(output_path.to_string());

    log::info!("FFmpeg command: {} {}", ffmpeg_path.display(), args.join(" "));

    // Create the command
    let mut command = Command::new(&ffmpeg_path);
    command
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null()); // Ensure stdin is null

    // Hide CMD window on Windows
    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);

    let mut child = command.spawn().map_err(|e| format!("Failed to start FFmpeg: {}", e))?;

    // Get the total duration for progress calculation
    let total_duration = if duration_mode == Some("trim") && duration.is_some() {
        duration
    } else {
        match get_media_info(input_path).await {
            Ok(info) => info.duration,
            Err(_) => None,
        }
    };

    // Handle both stdout (progress) and stderr (logs) in separate threads
    let stdout_handle = if let Some(stdout) = child.stdout.take() {
        let app_handle_clone = app_handle.clone();
        Some(thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                // Check for cancellation
                {
                    let cancelled = CONVERSION_CANCELLED.lock().unwrap();
                    if *cancelled {
                        break;
                    }
                }

                if let Ok(line_content) = line {
                    log::debug!("FFmpeg stdout: {}", line_content);
                    
                    if let Some(progress) = parse_progress_line(&line_content, total_duration) {
                        let _ = app_handle_clone.emit("conversion-progress", progress);
                    }

                    if line_content.contains("progress=end") {
                        log::info!("Conversion completed successfully");
                        break;
                    }
                }
            }
        }))
    } else {
        None
    };

    let stderr_handle = if let Some(stderr) = child.stderr.take() {
        Some(thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line_content) = line {
                    log::debug!("FFmpeg stderr: {}", line_content);
                }
            }
        }))
    } else {
        None
    };

    // Wait for the process to complete
    let exit_status = child.wait().map_err(|e| format!("Failed to wait for FFmpeg: {}", e))?;

    // Wait for the threads to complete
    if let Some(handle) = stdout_handle {
        let _ = handle.join();
    }
    if let Some(handle) = stderr_handle {
        let _ = handle.join();
    }

    // Check for cancellation
    {
        let cancelled = CONVERSION_CANCELLED.lock().unwrap();
        if *cancelled {
            // Clean up output file if it exists
            let _ = std::fs::remove_file(output_path);
            return Err("Conversion cancelled by user".to_string());
        }
    }

    if !exit_status.success() {
        log::error!("FFmpeg failed with exit code: {:?}", exit_status.code());
        return Err(format!("FFmpeg conversion failed with exit code: {:?}", exit_status.code()));
    }

    // Verify output file exists and has content
    if let Ok(metadata) = std::fs::metadata(output_path) {
        if metadata.len() == 0 {
            return Err("Output file is empty - conversion may have failed".to_string());
        }
        log::info!("Conversion completed successfully. Output file size: {} bytes", metadata.len());
    } else {
        return Err("Output file was not created".to_string());
    }

    log::info!("Media conversion completed successfully");
    Ok(())
}

async fn convert_image_simple(
    input_path: &str,
    output_path: &str,
    quality: Option<&str>,
    width: Option<u32>,
    height: Option<u32>,
) -> Result<(), String> {
    let ffmpeg_path = get_ffmpeg_path()?;
    
    let mut args = vec!["-i".to_string(), input_path.to_string()];
    
    let output_ext = std::path::Path::new(output_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if output_ext == "webp" {
        args.extend(["-c:v".to_string(), "libwebp".to_string()]);
        let quality_val = match quality {
            Some("low") => "60",
            Some("medium") => "80",
            Some("high") => "95",
            Some(q) if q.parse::<u8>().is_ok() => q,
            _ => "80",
        };
        args.extend(["-quality".to_string(), quality_val.to_string()]);
        
        // Add scaling for WebP if dimensions are specified
        if let (Some(w), Some(h)) = (width, height) {
            args.extend(["-vf".to_string(), format!("scale={}:{}", w, h)]);
        }
    } else if output_ext == "ico" {
        // For ICO format, we need to specify multiple sizes for best compatibility
        // ICO files typically contain multiple resolutions
        if let (Some(w), Some(h)) = (width, height) {
            // Use the specified dimensions
            args.extend(["-vf".to_string(), format!("scale={}:{}", w, h)]);
        } else {
            // Default to common icon sizes if no dimensions specified
            args.extend(["-vf".to_string(), "scale=32:32".to_string()]);
        }
        // PNG compression is good for ICO files
        args.extend(["-c:v".to_string(), "png".to_string()]);
    } else {
        // For other formats, just add scaling if dimensions are specified
        if let (Some(w), Some(h)) = (width, height) {
            args.extend(["-vf".to_string(), format!("scale={}:{}", w, h)]);
        }
    }

    args.extend(["-y".to_string(), output_path.to_string()]);

    let mut cmd = Command::new(&ffmpeg_path);
    cmd.args(&args).stdout(Stdio::piped()).stderr(Stdio::piped());
    
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    
    let output = cmd.output().map_err(|e| format!("Failed to execute FFmpeg: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Image conversion failed: {}", stderr));
    }

    Ok(())
}

fn parse_progress_line(line: &str, total_duration: Option<f64>) -> Option<ConversionProgress> {
    let mut progress = ConversionProgress {
        frame: None,
        fps: None,
        bitrate: None,
        total_size: None,
        out_time_us: None,
        speed: None,
        progress: None,
        percentage: None,
    };

    // Parse key=value pairs from FFmpeg progress output
    for part in line.split_whitespace() {
        if let Some((key, value)) = part.split_once('=') {
            match key {
                "frame" => {
                    progress.frame = value.parse().ok();
                }
                "fps" => {
                    progress.fps = value.parse().ok();
                }
                "bitrate" => {
                    progress.bitrate = Some(value.to_string());
                }
                "total_size" => {
                    progress.total_size = value.parse().ok();
                }
                "out_time_us" => {
                    progress.out_time_us = value.parse().ok();
                }
                "speed" => {
                    if let Ok(speed_val) = value.trim_end_matches('x').parse::<f64>() {
                        progress.speed = Some(speed_val);
                    }
                }
                "progress" => {
                    progress.progress = Some(value.to_string());
                    
                    // Calculate percentage if we have duration
                    if let (Some(out_time_us), Some(duration)) = (progress.out_time_us, total_duration) {
                        let current_time = out_time_us as f64 / 1_000_000.0; // Convert to seconds
                        let percentage = (current_time / duration * 100.0).min(100.0);
                        progress.percentage = Some(percentage);
                    }
                }
                _ => {}
            }
        }
    }

    // Only return progress if we have meaningful data
    if progress.frame.is_some() || progress.progress.is_some() {
        Some(progress)
    } else {
        None
    }
}

pub fn cancel_conversion() -> Result<(), String> {
    let mut cancelled = CONVERSION_CANCELLED.lock().unwrap();
    *cancelled = true;
    log::info!("Conversion cancellation requested");
    Ok(())
}

pub fn get_supported_formats() -> Vec<&'static str> {
    vec![
        // Video formats
        "mp4", "avi", "mov", "mkv", "webm", "flv", "wmv", "m4v",
        // Audio formats
        "mp3", "wav", "flac", "aac", "ogg", "wma", "m4a",
        // Image formats
        "jpg", "jpeg", "png", "webp", "bmp", "gif", "tiff", "ico",
    ]
}

pub async fn convert_image_to_webp(
    input_path: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let ffmpeg_path = get_ffmpeg_path()?;
    
    // Get the input file name without extension
    let input_file = std::path::Path::new(input_path);
    let file_stem = input_file.file_stem()
        .ok_or("Invalid input file path")?
        .to_string_lossy();
    
    // Get user's downloads folder
    let downloads_dir = get_downloads_directory()?;
    let output_path = downloads_dir.join(format!("{}.webp", file_stem));
    
    // Build FFmpeg command for image conversion
    let args = vec![
        "-i".to_string(),
        input_path.to_string(),
        "-c:v".to_string(),
        "libwebp".to_string(),
        "-quality".to_string(),
        "80".to_string(),
        "-y".to_string(),
        output_path.to_string_lossy().to_string(),
    ];

    // Start FFmpeg process
    let mut cmd = Command::new(&ffmpeg_path);
    cmd.args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    
    // Hide CMD window on Windows
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    
    let output = cmd.output()
        .map_err(|e| format!("Failed to start FFmpeg process: {}", e))?;

    if output.status.success() {
        let output_path_str = output_path.to_string_lossy().to_string();
        let _ = app_handle.emit("image-conversion-complete", &output_path_str);
        Ok(output_path_str)
    } else {
        let error_output = String::from_utf8_lossy(&output.stderr);
        Err(format!("FFmpeg image conversion failed: {}", error_output))
    }
}

fn get_downloads_directory() -> Result<std::path::PathBuf, String> {
    // Get user's downloads directory
    let home_dir = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map_err(|_| "Could not determine home directory")?;
    
    let downloads_dir = std::path::Path::new(&home_dir).join("Downloads");
    
    // Create downloads directory if it doesn't exist
    if !downloads_dir.exists() {
        std::fs::create_dir_all(&downloads_dir)
            .map_err(|e| format!("Failed to create downloads directory: {}", e))?;
    }
    
    Ok(downloads_dir)
}