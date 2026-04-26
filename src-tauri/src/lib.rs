mod commands;

use commands::analyze::analyze_screenshot;
use commands::capture::capture_screen_region;
use commands::clipboard::copy_to_clipboard;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

const CAPTURE_WINDOW_LABEL: &str = "capture";

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            capture_screen_region,
            analyze_screenshot,
            copy_to_clipboard,
            open_capture_overlay
        ])
        .setup(|app| {
            create_tray(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running ace");
}

fn create_tray(app: &tauri::App) -> tauri::Result<()> {
    let capture = MenuItem::with_id(app, "capture", "📸 Capture", true, None::<&str>)?;
    let last_answer = MenuItem::with_id(app, "last-answer", "📋 Last Answer", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "❌ Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&capture, &last_answer, &quit])?;

    let mut builder = TrayIconBuilder::new()
        .tooltip("ace")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "capture" => {
                let _ = app.emit("start-capture", ());
            }
            "last-answer" => {
                let _ = app.emit("show-answer", ());
            }
            "quit" => app.exit(0),
            _ => {}
        });

    let icon_bytes = include_bytes!("../icons/tray-icon.png");
    let icon = tauri::image::Image::from_bytes(icon_bytes)?;
    builder = builder.icon(icon);

    builder.build(app)?;
    Ok(())
}

#[tauri::command]
fn open_capture_overlay(app: tauri::AppHandle) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window(CAPTURE_WINDOW_LABEL) {
        let _ = window.close();
    }

    WebviewWindowBuilder::new(
        &app,
        CAPTURE_WINDOW_LABEL,
        WebviewUrl::App("index.html?mode=capture".into()),
    )
    .title("ace capture")
    .decorations(false)
    .transparent(true)
    .fullscreen(true)
    .always_on_top(true)
    .skip_taskbar(true)
    .focused(true)
    .build()?;

    Ok(())
}
