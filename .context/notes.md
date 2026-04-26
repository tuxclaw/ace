# Notes — ace (AI Screenshot Answer Tool)

## Architecture
- Tauri 2 desktop app with React frontend
- Rust backend handles: screenshot capture, global hotkey, window management
- React frontend handles: region selection overlay, answer display panel
- Google Gemini API for vision analysis (`gemini-3-flash-preview`)

## Bazzite Build Deps
- `xcap` has been removed; ace no longer needs direct X11/XWayland screenshot access or PipeWire development headers for app code.
- Screenshot capture now goes through `xdg-desktop-portal` via `ashpd`, so KDE's portal backend handles Wayland-safe capture.

## Wayland Screenshot Notes
- KDE Wayland blocks direct X11-style screen capture; use the portal path instead of trying to set `DISPLAY`/`XAUTHORITY`.
- Required user services: `xdg-desktop-portal`, `xdg-desktop-portal-kde`, and PipeWire should be active in the desktop session.
- Portal screenshot requests may show a desktop permission prompt depending on KDE policy.

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

## Google Gemini API Key
- Located at `/home/tux/Downloads/JacksKeys/Google.txt`
- Vision model: `gemini-3-flash-preview` via Google Generative Language API

## Style
- Dark theme (GitHub dark aesthetic, consistent with forge)
- Floating panel, minimal UI
- Monospace font for answer display
