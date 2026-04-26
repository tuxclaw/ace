# Decisions — ace (AI Screenshot Answer Tool)

## [2026-04-26] Initial Architecture
**By:** Jack (orchestrator)
**Context:** Tux wants a screenshot tool that captures screen regions and uses AI vision to answer questions (multiple choice, etc.)
**Decisions:**
- **Stack:** Tauri 2 + React + TypeScript + Tailwind (same as forge/ao-tauri)
- **Name:** ace
- **AI backend:** Venice API with vision model for screenshot analysis
- **Two-step pipeline:** Vision model reads screenshot → reasoning model answers question
- **Capture method:** Rust `xcap` crate for cross-platform screenshot capture
- **Hotkey:** `tauri-plugin-global-shortcut` for system-wide capture trigger
- **Overlay:** Transparent fullscreen Tauri window for region selection
- **Answer display:** Floating always-on-top panel with answer + explanation
- **History:** SQLite for storing past questions and answers
- **No backend server:** All local, direct API calls to Venice

## [2026-04-26] MVP Scope
**By:** Jack (orchestrator)
**Context:** First version should be simple and functional
**Decisions:**
- MVP = hotkey capture → vision analysis → answer display
- No history/database in MVP — just show the answer
- Single hotkey, single AI backend (Venice)
- Copy answer to clipboard on click
- Dark theme (consistent with forge/ao-tauri)

## [2026-04-26] MVP Implementation Details
**By:** Dash (subagent)
**Context:** Built the initial Tauri MVP.
**Decisions:**
- Kept Venice API calls in Rust so the frontend does not need to read or expose the API key; `analyze_screenshot` accepts an optional key and falls back to `/home/tux/Downloads/JacksKeys/Venice.txt` when blank.
- Registered `Ctrl+Shift+A` from Rust using `tauri-plugin-global-shortcut`; no JavaScript shortcut dependency is needed for the MVP.
- Used a hidden main window for the answer panel and a programmatic fullscreen transparent `capture` window for region selection.
- Pinned `xcap` to `=0.8.2` to match the requested screenshot dependency line closely and avoid silent minor-version drift.
- Set the Vite production target to `es2022`; the default Safari 13-style target failed esbuild transpilation with current React/Tauri dependencies.
