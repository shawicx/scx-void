# CLI 命令参考

## `void project` — 项目管理

| 子命令 | 说明 |
|--------|------|
| `project init` | 交互式创建项目 |
| `project ai-rule` | 管理 AI 规则文件 |

## `void system` — 系统操作

| 子命令 | 说明 |
|--------|------|
| `system shutdown` | 关闭系统（支持 `--timer` 倒计时） |

## `void setup` — 环境安装

| 子命令 | 说明 |
|--------|------|
| `setup frontend` | 交互式安装前端开发环境 |

### `setup frontend` 详情

启动交互式多选界面，从以下 9 个组件中选择安装：

**工具链（按依赖顺序）：**
1. fnm (Fast Node Manager)
2. Node.js (LTS)
3. pnpm
4. Bun
5. chsrc (镜像源切换)

**编辑器（独立）：**
6. Cursor
7. Visual Studio Code
8. WebStorm
9. Zed（仅 macOS）

流程：多选 → 确认 → 逐个安装 → 结果摘要。

## `void audio` — 音频转写

> 需要 `audio` feature：`cargo build --features audio`

| 子命令 | 说明 |
|--------|------|
| `audio transcribe` | 转写音频文件 |
| `audio download-model` | 下载 Whisper 模型 |
