# CLI 命令参考

scx-void 提供三大命令组：`project`、`system`、`audio`（可选）。

## 全局用法

```bash
scx-void <command> [subcommand] [options]
```

---

## project — 项目管理

### `project init`

从 GitHub 模板仓库初始化新项目，支持交互式和命令行两种模式。

**关联代码**：`src/cli/project.rs`

```bash
# 交互式模式（无参数时自动进入）
scx-void project init

# 命令行模式 — 预定义模板
scx-void project init --template node-ts-cli --name my-project

# 命令行模式 — 自定义仓库
scx-void project init --url shawicx/scx-template-cli --branch main --name my-project
```

**参数**：

| 参数 | 说明 | 必填 |
|------|------|------|
| `--name` | 项目名称 | 命令行模式必填 |
| `--template` | 预定义模板 ID | 与 `--url` 二选一 |
| `--url` | 自定义 GitHub 仓库（owner/repo） | 与 `--template` 二选一 |
| `--branch` | Git 分支（默认 main） | 否 |

**预定义模板 ID**：

| ID | 说明 |
|----|------|
| `node-ts-cli` | Node TypeScript CLI |
| `vue3-standard` | Vue 3 + TypeScript + Vite |
| `react-modern` | React 18 + TypeScript + Vite |
| `nestjs-rest` | NestJS REST API |
| `nextjs-app` | NextJS 14 + App Router + TypeScript |

**交互式模式**：

无参数运行时，通过 `dialoguer` 提示用户依次选择：
1. 项目名称
2. 模板列表（含"自定义仓库"选项）
3. 分支名（选择自定义仓库时）

### `project ai-rule`

生成或更新 `AGENTS.md` 文件，为 AI 编码助手提供项目规范。

**关联代码**：`src/cli/project.rs`

```bash
# 使用 basic 模板生成
scx-void project ai-rule --template basic

# 使用 advanced 模板
scx-void project ai-rule --template advanced

# 强制覆盖已存在的文件
scx-void project ai-rule --template advanced --force
```

**参数**：

| 参数 | 说明 | 必填 |
|------|------|------|
| `--template` | 模板类型：`basic` / `advanced` | 是 |
| `--force` | 强制覆盖已有文件（自动创建备份） | 否 |

**行为说明**：
- 若 `AGENTS.md` 已存在且未指定 `--force`，返回错误
- 使用 `--force` 时自动备份原文件为 `AGENTS.md.bak`
- 模板内容来自 `assets/templates/ai_rule/`

---

## system — 系统操作

### `system shutdown`

定时关机。

**关联代码**：`src/cli/system.rs`

```bash
# 立即关机
scx-void system shutdown

# 60 秒后关机
scx-void system shutdown --timer 60
```

**参数**：

| 参数 | 说明 | 必填 |
|------|------|------|
| `--timer` | 延迟秒数 | 否（默认 0） |

**平台差异**：

| 平台 | 命令 |
|------|------|
| macOS | `sudo shutdown -h +<分钟>` |
| Windows | `shutdown /s /t <秒>` |

> macOS 会将秒数转换为分钟（向上取整）。

---

## audio — 音频转写（可选功能）

> 需启用 `audio` feature：`cargo build --features audio`

### `audio transcribe`

将音频文件转写为纯文本。

**关联代码**：`src/cli/audio.rs`

```bash
scx-void audio transcribe \
  --file recording.m4a \
  --model base \
  --language zh
```

**参数**：

| 参数 | 说明 | 必填 |
|------|------|------|
| `--file` | 音频文件路径 | 是 |
| `--model` | Whisper 模型名称 | 是 |
| `--language` | 语言代码（如 `zh`、`en`） | 否 |

### `audio transcribe-with-timestamps`

转写音频并输出 SRT 格式时间戳。

```bash
scx-void audio transcribe-with-timestamps \
  --file recording.m4a \
  --model base \
  --language zh \
  --skip-seconds 10 \
  --end-time 120
```

**额外参数**：

| 参数 | 说明 | 必填 |
|------|------|------|
| `--skip-seconds` | 跳过前 N 秒 | 否 |
| `--end-time` | 截止时间（秒） | 否 |
| `--temperature` | 采样温度（0.0-1.0） | 否 |
| `--beam-size` | Beam search 宽度 | 否 |
| `--no-speech-threshold` | 静音过滤阈值 | 否 |

### `audio download-model`

下载 Whisper 模型到本地缓存。

```bash
scx-void audio download-model --model base
```

模型存储位置：`~/.scx-void/models/`

### `audio list-models`

列出所有可用模型及其下载状态。

```bash
scx-void audio list-models
```

---

## 相关文档

- [架构概览](../Overview/Architecture.md)
- [项目服务](../services/project/Project-Service.md)
- [音频服务](../services/audio/Audio-Service.md)
- [系统服务](../services/system/System-Service.md)

---

> 最后更新：2026-04-26
