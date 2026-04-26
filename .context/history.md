# History — ace (AI Screenshot Answer Tool)

## [2026-04-26] Project Created
- Initial scope defined by Jack and Tux
- Name: ace
- Stack: Tauri 2 + React + TypeScript + Tailwind
- Dispatched Dash to build MVP frontend

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
