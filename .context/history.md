# History — ace (AI Screenshot Answer Tool)

## [2026-04-26] Project Created
- Initial scope defined by Jack and Tux
- Name: ace
- Stack: Tauri 2 + React + TypeScript + Tailwind
- Dispatched Dash to build MVP frontend

## [2026-04-26] MVP Build — Dash
**Agent:** Dash ⚡ (GPT-5.5)
**Branch:** andy/ace-mvp
**Commit:** af37991 — Build ace MVP — AI screenshot answer tool
**Files:** Full Tauri 2 scaffold with Rust backend + React frontend
**Notes:** Initial build used Venice API with qwen3-vl-235b-a22b

## [2026-04-26] Switch to Gemini 3 Flash — Dash
**Agent:** Dash ⚡ (GPT-5.5)
**Branch:** andy/ace-mvp
**Commit:** 149b101 — Switch to Google Gemini 3 Flash for vision analysis
**Notes:** Tux prefers Gemini 3 Flash for vision. Uses generateContent API with inlineData.

## [2026-04-26] Rust Build Fixed — Jack
**Agent:** Jack (manual)
**Notes:** Bazzite /usr is read-only. Extracted pipewire-devel RPM to /tmp, set PKG_CONFIG_PATH/CPATH/LIBRARY_PATH. Also needed libgbm.so symlink. Cargo build succeeds.

## [2026-04-26] Helectrix Review
**Agent:** Helectrix ⚡
**Verdict:** ✅ Pass (2 minor warnings, 1 false-positive blocker)
**Notes:** Duplicate invoke import (cosmetic), unused apiKey param (cosmetic), API key path (correct — JacksKeys/Google.txt)

## [2026-04-26] Deployed
**Repo:** https://github.com/tuxclaw/ace
**Branch:** andy/ace-mvp

## [2026-04-26] MVP Built
- Built Tauri 2 + React 19 + TypeScript + Tailwind app structure.
- Added Rust commands for region screenshot capture, Venice vision analysis, and clipboard copy.
- Added global hotkey setup for `Ctrl+Shift+A` and programmatic transparent capture overlay window.
- Added React capture overlay, loading state, answer panel, event bridge, and copy-to-clipboard flow.
- Added Tauri config, capability permissions, generated icon, npm package setup, and git ignore rules.
- Verification: `npm run build` succeeds.
- Blocked verification: `cargo build` currently fails before compiling app code because the host is missing PipeWire development pkg-config files required by `xcap` (`libpipewire-0.3.pc`). Install/provide `pipewire-devel`/equivalent to complete Rust build verification.

## [2026-04-26] Switched Vision Analysis to Google Gemini 3 Flash
- Replaced the Rust Venice chat-completions request with Google Gemini `gemini-3-flash-preview:generateContent`.
- Changed auth from `Authorization: Bearer` to `x-goog-api-key`, falling back to `/home/tux/Downloads/JacksKeys/Google.txt` when no key is passed from Tauri.
- Updated request payload to Gemini `contents[].parts[]` with PNG `inlineData`, and response parsing to `candidates[0].content.parts[0].text`.
- Renamed the frontend hook from Venice-specific naming to `useVisionAPI` without changing the invoke contract.
- Verification: `npm run build` succeeds; `cargo build` still blocked by missing host PipeWire development pkg-config files required by `xcap`.

## [2026-04-26] Tray UX Pivot — Dash
**Agent:** Dash ⚡ (GPT-5.5)
**Branch:** andy/ace-tray
**Changes:** Replaced `Ctrl+Shift+A` global shortcut with a Tauri 2 system tray menu. Added tray actions for Capture, Last Answer, and Quit; Capture emits `start-capture` for the React app, which opens the existing transparent selection overlay via a Rust command. Last Answer emits `show-answer` and reopens the existing panel only when an answer is available.
**Rust:** Removed `tauri-plugin-global-shortcut`, enabled Tauri `tray-icon`, added tray menu setup, and exposed the existing overlay creation as `open_capture_overlay`.
**Frontend:** Added listeners for `start-capture` and `show-answer`; Gemini screenshot analysis flow unchanged.
**Verification:** `npm run build` passes; `cargo build` passes. `cargo fmt --check` could not run because rustfmt/cargo-fmt is not installed on this host.

## [2026-04-26] Tray UX Pivot — Dash
**Agent:** Dash ⚡ (GPT-5.5)
**Branch:** andy/ace-tray
**Commit:** 576faa4 — Switch from global hotkey to system tray with dropdown menu
**Changes:** Removed global-shortcut plugin, added tray-icon feature, tray menu with Capture/Last Answer/Quit, wired events to React listeners
**Review:** Helectrix ✅ Pass (2 warnings, 1 suggestion — no blockers)
**Build:** Bob ⚡ — 22s compile, 17.2 MB binary, MD5 d406488

## [2026-04-26] Deployed (tray version)
**Binary:** ~/.local/bin/ace
**MD5:** d406488ebddd7270d7357fd09aa9aa74
