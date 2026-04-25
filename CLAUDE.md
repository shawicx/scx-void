# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

```bash
cargo build                        # Debug build
cargo build --features audio       # Build with audio transcription support
cargo build --release              # Release build

cargo test                         # Run all tests
cargo test <test_name>             # Run single test (e.g. cargo test test_create_project)
cargo test --test <test_file>      # Run integration test file (e.g. cargo test --test system_commands_test)
cargo test -- --nocapture          # Show test output

cargo clippy -- -D warnings        # Lint (warnings as errors)
cargo fmt --check                  # Check formatting
cargo doc                          # Generate docs

cargo run -- <args>                # Run CLI (e.g. cargo run -- project init)
cargo install --path .             # Install locally
```

## Architecture

Three-layer architecture with strict unidirectional dependencies:

```
CLI (src/cli/)  →  Services (src/services/)  →  Platform (src/platform/) + Utils (src/utils/)
```

- **CLI layer**: `clap` subcommands (`project`, `system`, `audio`). Dispatches to services.
- **Services layer**: Business logic in `services/project/`, `services/audio/`, `services/system/`.
- **Platform/Utils layer**: Platform abstractions via `SystemOps` trait (`#[cfg(target_os)]` compile-time selection), plus `fs`, `git`, `shell` utilities.

Key patterns:
- **Centralized errors**: All error variants in `src/errors.rs` via `thiserror`. Never use `unwrap()` — always `?` or explicit handling.
- **Plugin-style extension**: New project types go in `installers/` and `templates/` without modifying existing code.
- **Feature flags**: Audio functionality gated behind `audio` feature (`whisper-rs`, `symphonia`).

Code style: imports ordered `std → external crate → crate::*`, `snake_case` functions/variables, `CamelCase` types/variants. All UI strings in Chinese. See `AGENTS.md` for full style guide.

## Wiki Documentation (.wiki/)

The `.wiki/` directory contains project technical documentation organized to mirror `src/` module structure. **This is a mandatory workflow:**

1. **Before starting a feature**: Read the relevant `.wiki/` documents to understand current architecture and constraints.
2. **After completing a feature**: Update the corresponding `.wiki/` documents to reflect any changes made.

```
.wiki/
├── Home.md                         ← Index
├── Overview/                       ← Architecture, Error Handling, Dev Guide
├── cli/                            ← CLI Reference
├── services/{project,audio,system} ← Service module docs
├── platform/                       ← Platform abstraction docs
└── utils/                          ← Utilities docs
```
