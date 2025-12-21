# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build --release    # Build release binary
cargo run -- <ADDR>      # Run with wall connector address
cargo clippy             # Lint
cargo test               # Run tests
```

## Architecture

Single-binary CLI tool that queries Tesla Wall Connector's local HTTP API. Uses clap for argument parsing, reqwest (blocking) for HTTP requests, and serde for JSON deserialization.

Entry point: `src/main.rs`
