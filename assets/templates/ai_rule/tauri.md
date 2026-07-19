## Tauri 项目规则

Tauri 项目同时包含前端（JS/TS）与后端（Rust），两套依赖体系独立维护。

### 包管理与运行时

1. **前端必须使用 `pnpm`**，Rust 后端使用 `cargo`，不得混用
2. **Node.js 版本要求 22+**（前端构建工具链）
3. Rust edition 必须与 `src-tauri/Cargo.toml` 中声明的版本一致（通常为 2021）

### 项目结构约定

遵循 Tauri 标准结构，不得擅自调整：

- `src/` 或前端根目录 — 前端代码（Vue/React/Svelte，遵循项目现状）
- `src-tauri/`
  - `src/main.rs` — Rust 入口
  - `src/lib.rs` — 应用逻辑（如使用 lib 结构）
  - `Cargo.toml` — Rust 依赖
  - `tauri.conf.json` — Tauri 配置（**含 security 字段**）
  - `icons/` — 应用图标
  - `capabilities/` — 权限配置（Tauri v2）

### 构建与测试命令

- `pnpm install` — 安装前端依赖
- `pnpm tauri dev` — 开发模式（同时启动前端与 Rust）
- `pnpm tauri build` — 生产打包
- `pnpm dev` — 仅启动前端（用于纯前端调试）
- `cargo test --manifest-path src-tauri/Cargo.toml` — Rust 测试

### 依赖与配置规则

1. **`tauri.conf.json` 的 security 相关字段必须保留**（CSP、allowlist、capabilities 等），不得删除或弱化
2. 前后端通过 Tauri IPC（`invoke` / `#[tauri::command]`）通信，命令必须在 `invoke_handler` 中注册
3. Rust 依赖遵循 cargo semver，不得降级
4. 前端依赖遵循 pnpm + Node 22+ 规范
5. `tauri.conf.json` 修改必须采取合并策略，不得重写整个文件
6. Tauri v2 项目权限必须在 `capabilities/` 下显式声明，不得通过 wildcard 放开

### 禁止事项

- 不得删除 `tauri.conf.json` 中的 security / CSP / capabilities 字段
- 不得在前端代码中直接调用 Rust crate（必须通过 IPC）
- 不得在 Rust 后端中改变 edition 版本（如从 2021 改为 2018）
- 不得擅自升级 Tauri 主版本（v1 ↔ v2 不兼容，迁移需用户明确批准）
- 不得破坏 `invoke_handler` 与前端 `invoke` 调用的对应关系
