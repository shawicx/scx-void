# scx-void

一款面向前端开发者的 Rust CLI 工具，提供项目初始化、环境搭建、AI 规则管理、系统操作等能力。

## 安装

```bash
# 基础安装
cargo install --path .

# 启用音频转录功能
cargo install --path . --features audio
```

或直接运行：

```bash
cargo run -- <command>
```

## 命令

### `project init` — 初始化项目

从模板或自定义 GitHub 仓库创建新项目。

```bash
# 交互式选择模板
scx-void project init

# 使用指定模板
scx-void project init -t vue3-standard

# 使用自定义仓库
scx-void project init -u owner/repo -n my-project

# 指定分支或标签
scx-void project init -t node-ts-cli -b v1.0
```

**参数：**

| 参数 | 缩写 | 说明 |
|------|------|------|
| `--template` | `-t` | 模板 ID（见下方模板列表） |
| `--url` | `-u` | 自定义 GitHub 仓库（格式：`owner/repo`） |
| `--branch` | `-b` | Git 分支或标签 |
| `--name` | `-n` | 项目名称 |

**可用模板：**

| ID | 名称 | 技术栈 |
|----|------|--------|
| `node-ts-cli` | Node TypeScript CLI | CLI 工具模板 |
| `vue3-standard` | Vue 3 Standard | Vue 3 + TypeScript + Vite |
| `react-modern` | React Modern | React 18 + TypeScript + Vite |
| `nestjs-rest` | NestJS REST API | NestJS RESTful API |
| `nextjs-app` | NextJS App Router | NextJS 14 + App Router + TypeScript |

### `project ai-rule` — 管理AI编码规则

生成 AI 代码规则文件（AGENTS.md）。

```bash
# 使用高级模板生成
scx-void project ai-rule

# 使用基础模板
scx-void project ai-rule -t basic

# 强制覆盖已有文件
scx-void project ai-rule -f

# 交互式配置
scx-void project ai-rule -i
```

**参数：**

| 参数 | 缩写 | 说明 |
|------|------|------|
| `--template` | `-t` | 模板类型：`basic` / `advanced`（默认：`advanced`） |
| `--force` | `-f` | 强制覆盖已有文件 |
| `--interactive` | `-i` | 交互式配置模式 |

### `setup frontend` — 搭建前端开发环境

交互式选择并安装前端开发工具。

```bash
scx-void setup frontend
```

**可选组件：**

- **fnm** — Node.js 版本管理器
- **Node.js** — 通过 fnm 安装最新 LTS 版本
- **pnpm** — 包管理器
- **Bun** — JavaScript 运行时与包管理器
- **chsrc** — 换源工具（中国大陆用户推荐）
- **Cursor** — 代码编辑器
- **VS Code** — 代码编辑器
- **WebStorm** — IDE（仅 macOS）
- **Zed** — 代码编辑器（仅 macOS）

### `system shutdown` — 系统关机

```bash
# 立即关机
scx-void system shutdown

# 定时关机（60秒后）
scx-void system shutdown -t 60
```

**参数：**

| 参数 | 缩写 | 说明 |
|------|------|------|
| `--timer` | `-t` | 延迟秒数（默认：0） |

### `audio` — 音频转录（需启用 audio feature）

```bash
# 需要启用 audio feature
cargo run --features audio -- audio <subcommand>
```

## 开发

```bash
cargo build                        # 调试构建
cargo build --features audio       # 启用音频功能
cargo build --release              # 发布构建

cargo test                         # 运行所有测试
cargo clippy -- -D warnings        # 代码检查
cargo fmt --check                  # 检查格式
```

## 许可证

MIT
