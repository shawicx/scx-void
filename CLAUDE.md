# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

```bash
# Build the project
cargo build

# Build for release
cargo build --release

# Run tests
cargo test

# Run the CLI with project init command
cargo run -- project init

# Run the CLI with system shutdown command
cargo run -- system shutdown --timer=30

# Check code formatting
cargo fmt --check

# Run clippy for linting
cargo clippy

# Install locally
cargo install --path .
```

## Project Architecture

**scx-void** is a Rust-based CLI tool for project management and system operations with a layered, plugin-based architecture designed for cross-platform compatibility and extensibility.

### Core Architecture Principles

1. **Plugin-style extensibility** - New project types and system commands can be added without modifying existing code
2. **Platform abstraction layer** - Cross-platform differences (shutdown commands, paths, etc.) are isolated in dedicated modules
3. **Clear boundary separation** - CLI handles interaction, Services handle logic, Platform handles system differences

### Module Structure

#### CLI Layer (`src/cli/`)
- **Purpose**: Command parsing, user interaction, and routing using `clap` and `dialoguer`
- **Commands**:
  - `project init` - Interactive project creation
  - `project add <type>` - Add technology to existing project
  - `system shutdown [--timer=seconds]` - Cross-platform system shutdown
- **Key files**: `project.rs`, `system.rs`, `utils.rs`

#### Services Layer (`src/services/`)
- **Project Service** (`src/services/project/`): Core project creation logic
  - `project_service.rs` - Main orchestration with validation
  - `generator.rs` - Template processing with variable injection
  - `installers/` - Technology-specific project creators
  - `templates/` - Template definitions
- **System Service** (`src/services/system/`): System-level operations
  - `shutdown.rs` - Cross-platform shutdown functionality

#### Platform Layer (`src/platform/`)
- **Pattern**: Trait-based abstraction using `SystemOps` trait
- **Implementations**:
  - `macos.rs` - macOS specific commands (e.g., `sudo shutdown -h +<minutes>`)
  - `windows.rs` - Windows specific commands (e.g., `shutdown /s /t <seconds>`)
- **Key trait**: `SystemOps` for cross-platform operations

#### Utils Layer (`src/utils/`)
- `fs.rs` - File system operations (create directories, write files, copy)
- `shell.rs` - Shell command execution using `duct` for cross-platform support

### Error Handling

- **Centralized errors** in `src/errors.rs` with `ScxVoidError` enum
- **Error categories**: General, Validation, FileSystem, Template errors
- **Result types**: All functions return `Result<T, ScxVoidError>`

### Project Type Extension Pattern

To add a new project type (e.g., "Go + Gin"):
1. Create `src/services/project/installers/go_gin.rs`
2. Implement the installer trait/pattern
3. Add entry to CLI selection in `project.rs`
4. Create template files in `assets/templates/go_gin/`

### Platform Extension Pattern

To add a new platform:
1. Create `src/platform/<platform>.rs`
2. Implement `SystemOps` trait methods
3. Add platform detection logic to `platform/mod.rs`

### Testing

- Tests in `tests/` directory using `assert_cmd` for CLI testing
- Integration tests verify cross-platform functionality
- Test file `tests/system_commands_test.rs` for system commands

### Current Implementation Status

**Implemented**:
- CLI structure with clap integration
- Project initialization with interactive prompts
- Cross-platform shutdown functionality
- File system utilities
- Error handling framework
- Basic Node TypeScript project template
- Test structure with `assert_cmd`

**In Progress**:
- Additional project type templates (React, Vue, NestJS, NextJS)
- Technology installer modules
- Advanced template processing with variable injection

**Planned**: Plugin system, template management commands, interactive project wizard, auto-update functionality

### Key Dependencies

- `clap` - CLI argument parsing
- `dialoguer` - Interactive user prompts
- `duct` - Cross-platform shell command execution
- `tokio` - Async runtime
- `thiserror` - Error handling
- `assert_cmd` - CLI testing