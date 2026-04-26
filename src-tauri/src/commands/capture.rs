use std::fs;
use std::io::Cursor;
use std::path::PathBuf;

use ashpd::desktop::screenshot::Screenshot;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use image::{GenericImageView, ImageFormat};

#[tauri::command]
pub async fn capture_screen_region(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    screen_x: u32,
    screen_y: u32,
) -> Result<String, String> {
    if width == 0 || height == 0 {
        return Err("Selection must have a width and height.".to_string());
    }

    let request = Screenshot::request()
        .interactive(false)
        .modal(false)
        .send()
        .await
        .map_err(|error| {
            format!("Failed to request screenshot through xdg-desktop-portal: {error}")
        })?;
    let screenshot = request
        .response()
        .map_err(|error| format!("Screenshot request was not completed: {error}"))?;
    let screenshot_path = file_uri_to_path(screenshot.uri().as_str())?;

    // Coordinates from the overlay are in CSS pixels relative to the overlay window.
    // On multi-monitor setups, the portal screenshot covers all monitors.
    // We need to detect the monitor offset and adjust.
    // For now, pass screen dimensions from frontend to help with offset detection.
    // screen_x/y is the overlay window's position on the virtual screen.
    // The selection coordinates are relative to the overlay, so we add the
    // screen offset to get the correct crop region on the full screenshot.
    let abs_x = x + screen_x;
    let abs_y = y + screen_y;

    tauri::async_runtime::spawn_blocking(move || {
        crop_and_encode_region(&screenshot_path, abs_x, abs_y, width, height)
    })
    .await
    .map_err(|error| format!("Screenshot encoding task failed: {error}"))?
}

fn crop_and_encode_region(
    screenshot_path: &PathBuf,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<String, String> {
    let image = image::open(screenshot_path)
        .map_err(|error| format!("Failed to read portal screenshot: {error}"))?;
    let _ = fs::remove_file(screenshot_path);

    let (image_width, image_height) = image.dimensions();
    if x >= image_width || y >= image_height {
        return Err(format!(
            "Selection starts outside screenshot bounds ({image_width}x{image_height})."
        ));
    }

    let crop_width = width.min(image_width - x);
    let crop_height = height.min(image_height - y);
    if crop_width == 0 || crop_height == 0 {
        return Err("Selection is outside the captured screenshot.".to_string());
    }

    let cropped = image.crop_imm(x, y, crop_width, crop_height);
    let mut png = Cursor::new(Vec::new());
    cropped
        .write_to(&mut png, ImageFormat::Png)
        .map_err(|error| format!("Failed to encode screenshot: {error}"))?;

    Ok(STANDARD.encode(png.into_inner()))
}

fn file_uri_to_path(uri: &str) -> Result<PathBuf, String> {
    let encoded_path = uri
        .strip_prefix("file://")
        .ok_or_else(|| format!("Portal returned a non-file screenshot URI: {uri}"))?;
    let decoded_path = percent_decode(encoded_path)?;
    Ok(PathBuf::from(decoded_path))
}

fn percent_decode(input: &str) -> Result<String, String> {
    let input = input.as_bytes();
    let mut decoded = Vec::with_capacity(input.len());
    let mut index = 0;

    while index < input.len() {
        if input[index] == b'%' {
            if index + 2 >= input.len() {
                return Err(
                    "Portal screenshot URI contains an incomplete percent escape.".to_string(),
                );
            }
            let hex = std::str::from_utf8(&input[index + 1..index + 3])
                .map_err(|error| format!("Invalid percent escape in screenshot URI: {error}"))?;
            let byte = u8::from_str_radix(hex, 16)
                .map_err(|error| format!("Invalid percent escape in screenshot URI: {error}"))?;
            decoded.push(byte);
            index += 3;
        } else {
            decoded.push(input[index]);
            index += 1;
        }
    }

    String::from_utf8(decoded)
        .map_err(|error| format!("Portal screenshot URI is not valid UTF-8: {error}"))
}
