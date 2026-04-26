# Decisions — ace (AI Screenshot Answer Tool)

## [2026-04-26] Initial Architecture
**By:** Jack (orchestrator)
**Context:** Tux wants a screenshot tool that captures screen regions and uses AI vision to answer questions (multiple choice, etc.)
**Decisions:**
- **Stack:** Tauri 2 + React + TypeScript + Tailwind (same as forge/ao-tauri)
- **Name:** ace
- **AI backend:** Google Gemini API (`gemini-3-flash-preview`) for screenshot analysis
- **Two-step pipeline:** Vision model reads screenshot → reasoning model answers question
- **Capture method:** `ashpd` + xdg-desktop-portal Screenshot API for Wayland-safe screen capture
- **Hotkey:** `tauri-plugin-global-shortcut` for system-wide capture trigger
- **Overlay:** Transparent fullscreen Tauri window for region selection
- **Answer display:** Floating always-on-top panel with answer + explanation
- **History:** SQLite for storing past questions and answers
- **No backend server:** All local, direct API calls to Gemini

## [2026-04-26] MVP Scope
**By:** Jack (orchestrator)
**Context:** First version should be simple and functional
**Decisions:**
- MVP = hotkey capture → vision analysis → answer display
- No history/database in MVP — just show the answer
- Single hotkey, single AI backend (Gemini)
- Copy answer to clipboard on click
- Dark theme (consistent with forge/ao-tauri)

## [2026-04-26] MVP Implementation Details
**By:** Dash (subagent)
**Context:** Built the initial Tauri MVP.
**Decisions:**
- Kept vision API calls in Rust so the frontend does not need to read or expose the API key; `analyze_screenshot` accepts an optional key and falls back to `/home/tux/Downloads/JacksKeys/Google.txt` when blank.
- Registered `Ctrl+Shift+A` from Rust using `tauri-plugin-global-shortcut`; no JavaScript shortcut dependency is needed for the MVP.
- Used a hidden main window for the answer panel and a programmatic fullscreen transparent `capture` window for region selection.
- Pinned `xcap` to `=0.8.2` to match the requested screenshot dependency line closely and avoid silent minor-version drift.
- Set the Vite production target to `es2022`; the default Safari 13-style target failed esbuild transpilation with current React/Tauri dependencies.

## [2026-04-26] UX Pivot — System Tray Instead of Global Hotkey
**By:** Jack (orchestrator)
**Context:** Global hotkey (Ctrl+Shift+A) didn't work reliably. Tux wants a system tray icon with dropdown menu instead — more discoverable, standard desktop pattern.
**Decision:**
- App launches to system tray (notification area icon)
- Click tray icon → dropdown menu with options
- Menu items: Capture, Last Answer, Quit (MVP)
- Capture → transparent overlay for region selection
- Remove global hotkey (replace with tray-triggered capture)
**Alternatives considered:**
- Global hotkey — unreliable, invisible, conflicts with other apps
- Standalone window — too heavy for a quick-answer tool
- Floating widget — always visible, but cluttered
**Status:** Active

## [2026-04-26] Screenshot Capture Switched to xdg-desktop-portal
**By:** Dash (subagent)
**Context:** `xcap` uses X11 and fails on KDE Wayland with I/O/connection errors even when XWayland is running.
**Decision:**
- Removed `xcap` and the `DISPLAY`/`XAUTHORITY` workaround.
- Added `ashpd` with the `screenshot` feature and direct `image` PNG support.
- Used the portal Screenshot API instead of raw ScreenCast/PipeWire because ace needs one still frame per region capture, not a continuous stream; this avoids hand-rolling PipeWire frame handling while still using the standard Wayland portal.
- Crop the selected region from the portal-returned PNG and base64-encode it for the existing Gemini pipeline.
**Status:** Active
