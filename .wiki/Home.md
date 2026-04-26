# scx-void 项目知识库

## 目录结构

```
.wiki/
├── Home.md                         ← 你在这里
├── Overview/                       ← 总览 & 基础设施
│   ├── Architecture.md
│   ├── Error-Handling.md
│   └── Development-Guide.md
├── cli/                            ← CLI 层
│   └── CLI-Reference.md
├── services/
│   ├── project/                    ← 项目管理模块
│   │   └── Project-Service.md
│   ├── audio/                      ← 音频转写模块
│   │   └── Audio-Service.md
│   └── system/                     ← 系统操作模块
│       └── System-Service.md
├── platform/                       ← 平台抽象层
│   └── Platform-Abstraction.md
└── utils/                          ← 工具函数层
    └── Utilities.md
```

---

## 文档索引

### Overview — 总览 & 基础设施

对应 `src/main.rs` · `src/errors.rs` · 项目全局

| 文档 | 说明 |
|------|------|
| [架构概览](Overview/Architecture.md) | 分层架构、模块关系、核心设计模式和依赖 |
| [错误处理](Overview/Error-Handling.md) | `ScxVoidError` 全部变体、使用规范和禁止事项 |
| [开发指南](Overview/Development-Guide.md) | 构建命令、测试、代码风格和扩展指南 |

### cli — CLI 层

对应 `src/cli/` — 命令行解析与交互入口

| 文档 | 说明 |
|------|------|
| [CLI 命令参考](cli/CLI-Reference.md) | `project` / `system` / `audio` 全部命令、参数和用法 |

### services/project — 项目管理模块

对应 `src/services/project/` — 项目创建、GitHub 模板下载、AI 规则

| 文档 | 说明 |
|------|------|
| [项目服务](services/project/Project-Service.md) | 创建流程、GitHub 归档下载、模板注册表、AI 规则管理 |

### services/audio — 音频转写模块

对应 `src/services/audio/` — Whisper 语音转写、解码、模型管理

| 文档 | 说明 |
|------|------|
| [音频服务](services/audio/Audio-Service.md) | 转写管线、AudioDecoder、ModelManager、WhisperTranscriber |

### services/system — 系统操作模块

对应 `src/services/system/` — 跨平台关机服务

| 文档 | 说明 |
|------|------|
| [系统服务](services/system/System-Service.md) | ShutdownService 和平台命令映射 |

### platform — 平台抽象层

对应 `src/platform/` — SystemOps trait 和编译时平台选择

| 文档 | 说明 |
|------|------|
| [平台抽象层](platform/Platform-Abstraction.md) | SystemOps trait、macOS/Windows 实现、扩展指南 |

### utils — 工具函数层

对应 `src/utils/` — 文件系统、GitHub URL 解析、Shell 工具

| 文档 | 说明 |
|------|------|
| [工具函数](utils/Utilities.md) | fs / git / shell 工具函数一览 |

---

> 最后更新：2026-04-26
