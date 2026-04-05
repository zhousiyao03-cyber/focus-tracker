# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

A macOS menubar app (Tauri 2 + React 19) that tracks which window/app is focused, sessionizes the data, and uploads it to a "Second Brain" backend. It runs as a tray-icon accessory app with a popover panel.

## Commands

```bash
pnpm install              # Install frontend dependencies
pnpm tauri dev --no-watch # Run the full app (Rust + React) in dev mode
pnpm build                # Build frontend only (tsc + vite)

cd src-tauri
cargo test                # Run Rust tests
cargo check               # Type-check Rust without building
```

Environment variables:
- `FOCUS_COLLECTOR_SAMPLE_INTERVAL_SECS` (default 5)
- `FOCUS_COLLECTOR_UPLOAD_INTERVAL_SECS` (default 120)
- `FOCUS_TRACKER_START_VISIBLE=true` to show panel on launch

## Architecture

**Rust backend** (`src-tauri/src/`): All core logic runs in Rust. A background thread loops every `sample_interval_secs`:

1. **tracker.rs** — Reads the frontmost app/window via `osascript` (AppleScript). Detects idle (System Events idle time) and away (screen locked / screensaver). Enriches samples with browser URL via accessibility APIs.
2. **browser_semantics.rs** — Extracts host, path, search query, surface type from browser URLs.
3. **accessibility.rs** — Reads browser address bar URL via macOS Accessibility API (AXUIElement).
4. **window_list.rs** — Gets visible windows via Core Graphics (`CGWindowListCopyWindowInfo`).
5. **sessionizer.rs** — Converts raw samples into sessions. Key behaviors: 3s switch confirmation delay, 5s minimum session duration, low-priority app filtering (Finder, etc. under 30s ignored). Stateful: holds `current` session and `pending` switch.
6. **outbox.rs** — Local JSON persistence for sessions. Dual queues: `queued_sessions` (pending upload) and `recent_sessions` (for local display). Deduplicates by `source_session_id`. Classifies apps into task groups (coding/design/meeting/communication/writing/research).
7. **uploader.rs** — Batch uploads queued sessions to `/api/focus/ingest` with bearer token auth.
8. **status_sync.rs** — Pulls canonical day status from `/api/focus/status` every 30s to reconcile local vs server data.
9. **pairing.rs** — Device pairing flow: exchange a code from the web app for a device token.
10. **state.rs** — `SharedState` (Mutex-wrapped `RuntimeState`) holds all runtime data. `TrackerStatus` is the serialized snapshot sent to the frontend. Settings and outbox are persisted to app data dir as JSON.
11. **error_state.rs** — Normalizes HTTP/network errors into user-facing messages.
12. **lib.rs** — Tauri app setup: tray icon, popover positioning, window management, background loop, all `#[tauri::command]` handlers.

**React frontend** (`src/App.tsx`): Single-file panel UI. Polls `get_status` every 5s via Tauri IPC (`invoke`). Shows working hours, focused time, 8h goal progress, 24h timeline, and setup/pairing flow.

**Data flow**: Sample → Sessionizer (debounce/filter) → Outbox (persist) → Uploader (batch POST) → Server. Server day snapshot is pulled back for display reconciliation.

## Tauri IPC Commands

`get_status`, `collect_once`, `flush_current_session`, `load_demo_fixture`, `upload_queue`, `pair_device`, `update_settings`, `hide_panel`, `set_panel_expanded`

## Key Design Decisions

- macOS only — uses `osascript`, `ioreg`, Core Graphics, Accessibility APIs
- App runs as `ActivationPolicy::Accessory` (no dock icon)
- Panel auto-hides on focus loss; positioned relative to tray icon with screen-edge clamping
- The Rust crate is named `focus_tracker_lib` (not `focus_tracker`) to avoid Windows cargo name collision
- Demo fixture is embedded at compile time via `include_str!` from `../../../tools/focus-collector/fixtures/`
