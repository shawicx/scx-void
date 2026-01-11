# 项目上下文

## 目的

scx-void 是一个多功能命令行工具（CLI），旨在简化开发者日常工作的常见任务。该工具主要提供以下功能：

1. **项目管理**：快速初始化和管理各种类型的项目（Vue、React、Next.js、NestJS、Node + TypeScript 等），支持本地模板和 Git 模板
2. **系统操作**：提供跨平台的系统操作功能（如定时关机）
3. **音频转录**：可选功能，使用 Whisper 模型进行音频文件转录（支持 M4A、AAC、MP4 格式）

## 技术栈

### 核心技术
- **Rust 2021 Edition**：主要开发语言
- **tokio 1.0**：异步运行时
- **clap 4.0**：命令行参数解析框架（derive 特性）

### 主要依赖
- **duct 0.13**：跨平台进程管理和命令执行
- **dialoguer 0.11**：交互式用户输入（菜单、确认等）
- **reqwest 0.11**：HTTP 客户端（支持流式下载）
- **dirs 5.0**：跨平台标准目录路径
- **indicatif 0.17**：进度条显示

### 可选功能（feature flags）
- **audio**：音频转录功能
  - **whisper-rs 0.10**：Whisper 语音识别模型绑定
  - **symphonia 0.5**：音频解码器（支持 AAC、M4A）

### 开发依赖
- **assert_cmd 2.0**：CLI 集成测试
- **predicates 3.0**：断言谓词
- **tempfile 3.10**：临时文件管理

## 项目约定

### 代码风格

#### 命名约定
- **函数/变量**：`snake_case`（如：`create_project`, `project_name`）
- **类型/结构体**：`CamelCase`（如：`ScxVoidError`, `ProjectCommands`）
- **枚举变体**：`CamelCase`（如：`GeneralError`, `InvalidProjectName`）
- **模块**：`snake_case`

#### 导入顺序
外部 crate（标准库优先）→ 内部模块（crate::*）

```rust
use std::path::Path;
use clap::{Parser, Subcommand};
use crate::errors::ScxVoidError;
```

#### 类型注解
- 公共 API 函数必须显式标注类型
- 本地变量使用类型推断
- 所有可能失败的操作返回 `Result<T, ScxVoidError>`

#### 注释风格
- 公共 API 使用 `///` 文档注释
- 行内注释使用 `//`
- 保持文件内语言一致（中文或英文）

#### 结构体和枚举
- 公共结构体/枚举使用 `#[derive(Debug)]`
- 未使用的代码标记 `#[allow(dead_code)]`
- 使用 `#[cfg(test)]` 组织测试

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

### 架构模式

#### 分层架构
```
CLI 层（src/cli/）
    ↓
Services 层（src/services/）
    ↓
Platform/Utils 层（src/platform/, src/utils/）
```

#### Trait 抽象
使用 trait 定义平台特定接口，确保跨平台兼容性：

```rust
pub trait SystemOps {
    fn shutdown_in(&self, seconds: u64) -> duct::Expression;
}
```

#### 插件式扩展
- 新项目类型添加到 `installers/` 目录
- 新模板添加到 `templates/` 目录
- 不修改现有代码

#### 集中化错误
所有错误类型定义在 `src/errors.rs`，统一错误处理：

```rust
#[derive(Debug)]
pub enum ScxVoidError {
    GeneralError(String),
    FileSystemError(String),
    // ...
}
```

### 测试策略

#### 单元测试
- 放在模块内的 `#[cfg(test)]` 块
- 测试名称使用 `test_` 前缀

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcription_segment() {
        let segment = TranscriptionSegment {
            start_ms: 1500,
            end_ms: 3500,
            text: "Hello world".to_string()
        };
        assert_eq!(segment.duration_ms(), 2000);
    }
}
```

#### 集成测试
- 放在 `tests/` 目录
- 使用 `assert_cmd` 进行 CLI 测试

```rust
use assert_cmd::Command;

#[test]
fn test_system_shutdown_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scx-void")?;
    let assert = cmd.arg("system").arg("shutdown").arg("--help").assert();
    assert.success().stdout(predicates::str::contains("--timer"));
    Ok(())
}
```

### Git工作流程

#### 分支策略
- `main/master`：稳定分支
- `feature/*`：功能开发分支
- `bugfix/*`：Bug 修复分支

#### 提交约定
使用 Conventional Commits 规范：
- `feat:`：新功能
- `fix:`：Bug 修复
- `docs:`：文档更新
- `refactor:`：代码重构
- `test:`：测试相关
- `chore:`：构建/工具相关

示例：
```
feat: 添加 Next.js 项目模板支持
fix: 修复 Whisper 模型下载超时问题
```

#### Pull Request 流程
1. 从 `main` 创建功能分支
2. 开发并测试
3. 运行 `cargo clippy` 和 `cargo test`
4. 创建 Pull Request
5. 代码审查
6. 合并到 `main`

## 领域上下文

### 项目类型支持
- **Vue**：Vue.js 项目（支持 Tailwind CSS）
- **React**：React 项目（支持 Tailwind CSS）
- **Next.js**：Next.js 全栈框架
- **NestJS**：Node.js 后端框架
- **Node + TypeScript**：TypeScript Node.js 项目

### 模板系统
- **本地模板**：内置的项目模板
- **Git 模板**：从 Git 仓库克隆的模板（支持分支选择）
- **模板注册表**：管理可用的 Git 模板

### 音频转录
- **支持格式**：M4A、AAC、MP4
- **模型**：Whisper（tiny、base、small、medium、large）
- **输出**：带时间戳的文本转录

## 重要约束

### 技术约束
- 最低 Rust 版本：1.70.0（2021 Edition）
- 需要系统安装 Git（用于 Git 模板功能）
- 音频功能需要系统有足够的 CPU/GPU 资源

### 平台支持
- **主要支持**：macOS、Windows、Linux
- **系统操作**：使用平台特定实现（条件编译）

### 安全约束
- 不在代码中硬编码敏感信息
- 下载的模型文件存储在用户目录下
- 不执行不受信任的外部命令

### 依赖管理
- 不降级依赖版本（除非用户明确要求）
- 所有外部 crate 必须在 `Cargo.toml` 中声明
- 保持依赖更新但谨慎处理 breaking changes

## 外部依赖

### Whisper 模型
- **来源**：Hugging Face 或 GitHub
- **模型大小**：
  - tiny: ~39 MB
  - base: ~74 MB
  - small: ~244 MB
  - medium: ~769 MB
  - large: ~1550 MB

### Git 仓库（用于模板）
- 支持所有 Git 协议（HTTPS、SSH、Git）
- 需要用户有访问权限

### 系统命令
- **macOS**：使用 `shutdown -h +<seconds>`
- **Windows**：使用 `shutdown /s /t <seconds>`
- **Linux**：使用 `shutdown -h +<seconds>`

## 开发命令

```bash
# 构建
cargo build                    # Debug 构建
cargo build --release          # Release 构建

# 测试
cargo test                     # 运行所有测试
cargo test <test_name>         # 运行单个测试
cargo test --test <test_file>  # 运行特定测试文件
cargo test -- --nocapture      # 显示测试输出

# Lint 和格式化
cargo clippy                   # 运行 Clippy linter
cargo clippy -- -D warnings    # 将警告视为错误
cargo fmt                      # 格式化代码
cargo fmt --check              # 检查格式化
cargo doc                      # 生成文档

# 运行
cargo run -- <args>            # 运行 CLI
cargo install --path .         # 本地安装
```

## 禁止事项

- 降级依赖版本（除非用户明确要求）
- 擅自修改项目原有功能
- 生成冗余的总结文档（除非用户要求）
- 使用 `unwrap()` 处理可能失败的操作（使用 `?` 或适当的错误处理）
- 硬编码路径（使用 `dirs` crate 或环境相关路径）
- 混用环境（浏览器项目不得插入 Node API，CLI 不得依赖浏览器 API）
