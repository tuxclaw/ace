use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use xcap::Monitor;

#[tauri::command]
pub async fn capture_screen_region(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<String, String> {
    if width == 0 || height == 0 {
        return Err("Selection must have a width and height.".to_string());
    }

    tauri::async_runtime::spawn_blocking(move || capture_region_blocking(x, y, width, height))
        .await
        .map_err(|error| format!("Screenshot task failed: {error}"))?
}

fn ensure_display_env() {
    if std::env::var("DISPLAY").unwrap_or_default().is_empty() {
        if let Ok(entries) = std::fs::read_dir("/run/user") {
            for user_dir in entries.flatten() {
                if let Some(uid) = user_dir.file_name().to_str() {
                    // Try :0 as default display
                    std::env::set_var("DISPLAY", ":0");
                    // Find xauth file
                    let xauth_dir = user_dir.path().join(uid);
                    if let Ok(xauth_entries) = std::fs::read_dir(&xauth_dir) {
                        for entry in xauth_entries.flatten() {
                            if let Some(name) = entry.file_name().to_str() {
                                if name.starts_with("xauth_") {
                                    std::env::set_var("XAUTHORITY", entry.path());
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn capture_region_blocking(x: u32, y: u32, width: u32, height: u32) -> Result<String, String> {
    ensure_display_env();
    let monitor = Monitor::from_point(x as i32, y as i32)
        .or_else(|_| primary_monitor())
        .map_err(|error| format!("No monitor available for screenshot capture: {error}"))?;

    let monitor_x = monitor.x().unwrap_or(0);
    let monitor_y = monitor.y().unwrap_or(0);
    let local_x = (x as i32 - monitor_x).max(0) as u32;
    let local_y = (y as i32 - monitor_y).max(0) as u32;
    let image = monitor
        .capture_region(local_x, local_y, width, height)
        .map_err(|error| format!("Failed to capture selected region: {error}"))?;

    let path = temp_png_path();
    image
        .save(&path)
        .map_err(|error| format!("Failed to encode screenshot: {error}"))?;
    let bytes = fs::read(&path).map_err(|error| format!("Failed to read encoded screenshot: {error}"))?;
    let _ = fs::remove_file(&path);

    Ok(STANDARD.encode(bytes))
}

fn primary_monitor() -> Result<Monitor, String> {
    let monitors = Monitor::all().map_err(|error| error.to_string())?;
    monitors
        .into_iter()
        .find(|monitor| monitor.is_primary().unwrap_or(false))
        .ok_or_else(|| "Primary monitor was not found.".to_string())
}

fn temp_png_path() -> std::path::PathBuf {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default();
    std::env::temp_dir().join(format!("ace-capture-{}-{millis}.png", std::process::id()))
}
