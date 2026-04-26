# Notes — ace (AI Screenshot Answer Tool)

## Architecture
- Tauri 2 desktop app with React frontend
- Rust backend handles: screenshot capture, global hotkey, window management
- React frontend handles: region selection overlay, answer display panel
- Venice API for vision analysis (gpt-image-2 or similar vision model)

## Key Technical Challenges
- **Region capture:** Need transparent fullscreen window, mouse drag to select region, crop image in Rust
- **Always-on-top answer panel:** Separate Tauri window, positioned top-right, stays on top
- **Hotkey conflict:** Must not conflict with other apps (Ctrl+Shift+A is a good choice)
- **Vision API latency:** Show loading state while AI processes screenshot

## Tauri Docs Available
- `docs/library/tauri/getting-started.md` — architecture overview
- `docs/library/tauri/calling-rust.md` — commands, state management
- `docs/library/tauri/window-customization.md` — transparent windows, drag regions
- `docs/library/tauri/system-tray.md` — tray icons (future)

## Venice API Key
- Located at `/home/tux/Downloads/JacksKeys/Venice.txt`
- Vision-capable models: check `docs/library/venice/models.md`

## Style
- Dark theme (GitHub dark aesthetic, consistent with forge)
- Floating panel, minimal UI
- Monospace font for answer display
