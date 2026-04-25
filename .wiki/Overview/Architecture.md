# 架构概览

scx-void 是一个多功能的 Rust CLI 工具，提供项目初始化、音频转写和系统操作等能力。

## 架构分层

项目采用**三层分层架构**，职责清晰分离：

```
┌─────────────────────────────────────────────────────────┐
│  CLI 层 (src/cli/)                                      │
│  解析命令行参数，处理用户交互，分发到 Services 层          │
├─────────────────────────────────────────────────────────┤
│  Services 层 (src/services/)                            │
│  核心业务逻辑：项目管理、音频转写、系统操作                │
├─────────────────────────────────────────────────────────┤
│  Platform / Utils 层 (src/platform/, src/utils/)        │
│  平台抽象和工具函数：文件系统、Git、Shell 操作            │
└─────────────────────────────────────────────────────────┘
```

### 层间依赖规则

- **CLI 层** → 调用 Services 层，不直接操作 Platform/Utils
- **Services 层** → 调用 Platform/Utils 完成底层操作，不依赖 CLI
- **Platform/Utils 层** → 独立的底层能力，无上层依赖

## 模块结构

```
src/
├── main.rs                  # 入口：CLI 定义和分发
├── errors.rs                # 集中化错误类型定义
├── cli/                     # CLI 层
│   ├── mod.rs
│   ├── project.rs           # project 命令（init / ai-rule）
│   ├── system.rs            # system 命令（shutdown）
│   ├── audio.rs             # audio 命令（transcribe / download-model）
│   └── utils.rs             # CLI 工具函数
├── services/                # Services 层
│   ├── project/             # 项目管理服务
│   │   ├── project_service.rs   # 项目创建主逻辑
│   │   ├── project.rs           # Project 服务结构体
│   │   ├── generator.rs         # 模板生成器
│   │   ├── ai_rule.rs           # AI 规则文件管理
│   │   ├── templates/           # 内置模板定义
│   │   │   ├── node_ts.rs / react.rs / vue.rs
│   │   ├── installers/          # 项目安装器
│   │   │   ├── node_ts.rs / react.rs / vue.rs / nestjs.rs / nextjs.rs / tailwind.rs
│   │   └── git/                 # Git 模板系统
│   │       ├── types.rs         # 数据类型
│   │       ├── registry.rs      # 模板注册表
│   │       ├── downloader.rs    # 模板下载器
│   │       ├── validator.rs     # 模板验证器
│   │       └── clone.rs         # Git 克隆
│   ├── audio/               # 音频转写服务
│   │   ├── audio_service.rs     # 转写服务主逻辑
│   │   ├── decoder.rs           # 音频解码器
│   │   ├── model.rs             # Whisper 模型管理
│   │   └── whisper.rs           # Whisper 集成
│   └── system/              # 系统操作服务
│       └── shutdown.rs          # 关机服务
├── platform/                # 平台抽象层
│   ├── mod.rs                   # SystemOps trait 定义
│   ├── macos.rs                 # macOS 实现
│   └── windows.rs               # Windows 实现
└── utils/                   # 工具函数层
    ├── fs.rs                    # 文件系统操作
    ├── git.rs                   # Git 工具
    └── shell.rs                 # Shell 工具
```

## 核心设计模式

### 1. Trait 抽象（平台无关性）

`SystemOps` trait 定义平台特定接口，编译时选择具体实现：

- 关联代码：`src/platform/mod.rs`

```rust
pub trait SystemOps {
    fn shutdown_in(&self, seconds: u64) -> duct::Expression;
}
```

→ 详见 [平台抽象层](../platform/Platform-Abstraction.md)

### 2. 插件式扩展

新增项目类型只需在 `installers/` 和 `templates/` 添加对应模块，无需修改现有代码。

### 3. Feature Flag

音频功能通过 Cargo feature gate 控制：

- 关联代码：`Cargo.toml`

```toml
[features]
default = []
audio = ["whisper-rs", "symphonia"]
```

### 4. 集中化错误处理

所有错误变体统一定义在 `src/errors.rs`，使用 `thiserror` 派生 `Display` 和 `Error` trait。

→ 详见 [错误处理](Error-Handling.md)

## 外部依赖关系

| 依赖 | 用途 | 是否可选 |
|------|------|----------|
| clap | CLI 参数解析 | 必选 |
| dialoguer | 交互式提示 | 必选 |
| duct | Shell 命令执行 | 必选 |
| tokio | 异步运行时 | 必选 |
| reqwest | HTTP 请求（模型下载） | 必选 |
| indicatif | 进度条 | 必选 |
| symphonia | 音频解码 | 可选（audio feature） |
| whisper-rs | Whisper 语音转文字 | 可选（audio feature） |
| thiserror | 错误类型派生 | 必选 |
| dirs | 系统目录路径 | 必选 |

## 相关文档

- [错误处理](Error-Handling.md)
- [开发指南](Development-Guide.md)
- [CLI 命令参考](../cli/CLI-Reference.md)
- [项目服务](../services/project/Project-Service.md)
- [音频服务](../services/audio/Audio-Service.md)
- [系统服务](../services/system/System-Service.md)
- [平台抽象层](../platform/Platform-Abstraction.md)
- [工具函数](../utils/Utilities.md)

---

> 最后更新：2026-04-25
