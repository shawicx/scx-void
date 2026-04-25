# 开发指南

## 环境要求

- **Rust** — 推荐 stable 版本
- **Cargo** — Rust 包管理器
- **Git** — 模板系统依赖

### 音频功能额外要求

- 需编译时启用 feature：`--features audio`
- Whisper 模型文件（通过 CLI 下载或手动放置到 `~/.scx-void/models/`）

---

## 构建与运行

```bash
# Debug 构建
cargo build

# Release 构建
cargo build --release

# 构建（含音频功能）
cargo build --features audio

# 运行
cargo run -- <command> [subcommand] [options]

# 本地安装
cargo install --path .
```

---

## 测试

```bash
# 运行所有测试
cargo test

# 运行单个测试
cargo test test_create_project

# 运行特定测试文件
cargo test --test system_commands_test

# 显示测试输出
cargo test -- --nocapture

# 运行测试（含音频功能）
cargo test --features audio
```

### 测试组织

| 位置 | 说明 |
|------|------|
| `#[cfg(test)] mod tests` | 单元测试（位于模块内部） |
| `tests/*.rs` | 集成测试 |

### 测试框架

- **assert_cmd** — CLI 命令测试
- **predicates** — 输出断言匹配

### 现有集成测试

| 文件 | 覆盖范围 |
|------|----------|
| `tests/system_commands_test.rs` | system 命令帮助文本和参数验证 |
| `tests/audio_transcribe_test.rs` | audio 命令帮助文本和错误处理 |
| `tests/ai_rule_test.rs` | AI 规则文件生成、覆盖和备份 |
| `tests/git_phase1_test.rs` | Git 模块基础验证 |

---

## 代码质量

```bash
# Clippy 静态检查
cargo clippy

# 严格模式（警告视为错误）
cargo clippy -- -D warnings

# 格式化
cargo fmt

# 检查格式
cargo fmt --check

# 生成文档
cargo doc
```

---

## 代码风格

### 导入顺序

```rust
use std::path::Path;                // 标准库
use clap::{Parser, Subcommand};     // 外部 crate
use crate::errors::ScxVoidError;    // 内部模块
```

### 命名约定

| 类型 | 风格 | 示例 |
|------|------|------|
| 函数 / 变量 | `snake_case` | `create_project`, `project_name` |
| 类型 / 结构体 | `CamelCase` | `ScxVoidError`, `ProjectCommands` |
| 枚举变体 | `CamelCase` | `GeneralError`, `InvalidProjectName` |
| 模块 | `snake_case` | `project_service`, `audio_service` |

### 注释

- 公共 API：`///` 文档注释
- 行内注释：`//`
- 保持文件内语言一致（中文或英文）

### 结构体和枚举

```rust
#[derive(Debug)]
pub enum ScxVoidError {
    GeneralError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {}
}
```

---

## 项目架构原则

1. **分层架构**：CLI → Services → Platform/Utils，严格单向依赖
2. **Trait 抽象**：平台特定操作通过 trait 定义接口
3. **插件式扩展**：新项目类型只需添加文件，不修改现有代码
4. **集中化错误**：所有错误类型定义在 `src/errors.rs`
5. **Feature Flag**：可选功能通过 Cargo feature 控制

→ 详见 [架构概览](Architecture.md)

---

## 新增项目类型指南

1. 在 `src/services/project/installers/` 创建安装器文件
2. 在 `src/services/project/templates/` 创建模板定义文件（如需要）
3. 在对应 `mod.rs` 中注册模块
4. 在 `project_service.rs` 添加类型索引映射
5. 在 `tests/` 添加集成测试
6. 运行 `cargo clippy` 和 `cargo test` 验证

---

## 相关文档

- [架构概览](Architecture.md)
- [错误处理](Error-Handling.md)
- [CLI 命令参考](../cli/CLI-Reference.md)

---

> 最后更新：2026-04-25
