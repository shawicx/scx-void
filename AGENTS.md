# AGENTS.md

本文件为在此 Rust 项目中工作的 AI 代理提供指导。

## 开发命令

```bash
# 构建
cargo build                    # Debug 构建
cargo build --release          # Release 构建

# 测试
cargo test                     # 运行所有测试
cargo test <test_name>         # 运行单个测试（如：cargo test test_create_project）
cargo test --test <test_file>  # 运行特定测试文件（如：cargo test --test system_commands_test）
cargo test -- --nocapture      # 显示测试输出

# Lint 和格式化
cargo clippy                   # 运行 Clippy linter
cargo clippy -- -D warnings    # 将警告视为错误
cargo fmt                      # 格式化代码
cargo fmt --check              # 检查格式化
cargo doc                      # 生成文档

# 运行
cargo run -- <args>            # 运行 CLI（如：cargo run -- project init）
cargo install --path .         # 本地安装
```

## 代码风格指南

### 导入顺序
外部 crate（标准库优先）→ 内部模块（crate::*）

```rust
use std::path::Path;
use clap::{Parser, Subcommand};
use crate::errors::ScxVoidError;
```

### 命名约定
- **函数/变量**: `snake_case`（如：`create_project`, `project_name`）
- **类型/结构体**: `CamelCase`（如：`ScxVoidError`, `ProjectCommands`）
- **枚举变体**: `CamelCase`（如：`GeneralError`, `InvalidProjectName`）
- **模块**: `snake_case`

### 类型注解
公共 API 函数必须显式标注类型，本地变量使用类型推断。所有可能失败的操作返回 `Result<T, ScxVoidError>`。

### 错误处理
使用 `?` 操作符进行错误传播，在 `src/errors.rs` 中定义新错误变体，使用 `.map_err()` 转换外部错误。

```rust
fs::create_dir(project_name).map_err(|e| {
    ScxVoidError::FileSystemError(format!("创建项目目录失败: {}", e))
})?;
```

### 注释风格
公共 API 使用 `///` 文档注释，行内注释使用 `//`，保持文件内语言一致（中文或英文）。

### 结构体和枚举
公共结构体/枚举使用 `#[derive(Debug)]`，未使用的代码标记 `#[allow(dead_code)]`，使用 `#[cfg(test)]` 组织测试。

```rust
#[derive(Debug)]
pub enum ScxVoidError {
    GeneralError(String),
    FileSystemError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_error_display() {}
}
```

### 条件编译
平台特定代码使用 `#[cfg(target_os = "...")]`，测试代码使用 `#[cfg(test)]`，避免运行时平台检测。

```rust
#[cfg(target_os = "macos")]
pub fn get_platform() -> impl SystemOps {
    MacosPlatform {}
}

#[cfg(target_os = "windows")]
pub fn get_platform() -> impl SystemOps {
    WindowsPlatform {}
}
```

### 异步代码
异步函数使用 `async` 关键字，调用时使用 `.await`，主函数使用 `#[tokio::main]`。

```rust
#[tokio::main]
async fn main() {
    create_project("myproject", 0).await;
}
```

### 模块组织
每个模块目录包含 `mod.rs`，子模块使用独立文件，使用 `pub use` re-export 公开重要类型，保持分层架构：CLI → Services → Platform/Utils。

```rust
// src/platform/mod.rs
pub mod windows;
pub mod macos;
use macos::MacosPlatform;

pub trait SystemOps {
    fn shutdown_in(&self, seconds: u64) -> duct::Expression;
}
```

### 测试规范
单元测试放在 `#[cfg(test)]` 模块，集成测试放在 `tests/` 目录，使用 `assert_cmd` 进行 CLI 测试，测试名称使用 `test_` 前缀。

```rust
#[test]
fn test_transcription_segment() {
    let segment = TranscriptionSegment { start_ms: 1500, end_ms: 3500, text: "Hello world".to_string() };
    assert_eq!(segment.duration_ms(), 2000);
}

// 集成测试（tests/system_commands_test.rs）
#[test]
fn test_system_shutdown_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scx-void")?;
    let assert = cmd.arg("system").arg("shutdown").arg("--help").assert();
    assert.success().stdout(predicates::str::contains("--timer"));
    Ok(())
}
```

## 架构原则

1. **分层架构**：CLI 层 → Services 层 → Platform/Utils 层
2. **Trait 抽象**：使用 trait 定义平台特定接口（如 `SystemOps`）
3. **插件式扩展**：新项目类型添加到 `installers/` 目录，不修改现有代码
4. **集中化错误**：所有错误类型定义在 `src/errors.rs`

## 禁止事项

- 降级依赖版本（除非用户明确要求）
- 擅自修改项目原有功能
- 生成冗余的总结文档（除非用户要求）
- 使用 `unwrap()` 处理可能失败的操作（使用 `?` 或适当的错误处理）
- 硬编码路径（使用 `dirs` crate 或环境相关路径）
- 混用环境（浏览器项目不得插入 Node API，CLI 不得依赖浏览器 API）

## 重要提醒

- 完成任务后运行 `cargo clippy` 和 `cargo test` 确保代码质量
- 保持与现有代码风格一致，遵循项目约定
- 输出简洁直接，除非用户要求解释或详细说明
- 所有外部 crate 需已在 `Cargo.toml` 中声明
