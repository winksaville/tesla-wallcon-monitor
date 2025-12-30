# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build --release    # Build release binary
cargo run -- <ADDR> <CMD> # Run with wall connector address and command
cargo clippy             # Lint
cargo test               # Run tests
```

## Architecture

Single-binary CLI tool (`src/main.rs`) that queries Tesla Wall Connector Gen 3's local HTTP API at `http://<addr>/api/1/<endpoint>`.

**Dependencies**: clap (CLI parsing with derive), reqwest (blocking HTTP), serde/serde_json (JSON), crossterm (terminal UI for loop mode), simplelog (file logging), base64 (SSID decoding).

**API Endpoints & Data Structures**:
- `/vitals` → `Vitals` struct - real-time charging data (currents, voltages, temps, session info)
- `/lifetime` → `Lifetime` struct - cumulative stats (energy delivered, charge starts, uptime)
- `/version` → `Version` struct - firmware and device info
- `/wifi_status` → `WifiStatus` struct - network connection details

**Command Matching**: Commands use prefix matching via `match_command()`. Minimum unique prefixes: `l`=lifetime, `ve`=version, `vi`=vitals, `w`=wifi_status.

**Loop Mode**: The `vitals` command supports `-l/--loop-mode` which uses crossterm's raw mode for continuous display updates with ESC/Ctrl+C to exit.
