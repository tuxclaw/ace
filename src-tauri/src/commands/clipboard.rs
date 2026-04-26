#[tauri::command]
pub async fn copy_to_clipboard(text: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let mut clipboard = arboard::Clipboard::new()
            .map_err(|error| format!("Failed to open clipboard: {error}"))?;
        clipboard
            .set_text(text)
            .map_err(|error| format!("Failed to copy answer: {error}"))
    })
    .await
    .map_err(|error| format!("Clipboard task failed: {error}"))?
}
