# 环境安装服务 (Setup Service)

对应 `src/services/setup/` — 前端开发环境一键安装

## 架构

```
SetupService (编排器)
  ├── Installer trait (统一接口)
  │   ├── FnmInstaller
  │   ├── NodeInstaller
  │   ├── PnpmInstaller
  │   ├── BunInstaller
  │   ├── ChsrcInstaller
  │   ├── CursorInstaller
  │   ├── VscodeInstaller
  │   ├── WebstormInstaller
  │   └── ZedInstaller (macOS only)
  └── ShellConfig (shell 环境配置工具)
```

## Installer Trait

```rust
pub trait Installer {
    fn name(&self) -> &str;                              // 中文显示名
    fn is_installed(&self) -> Option<String>;            // 检测安装状态
    fn install(&self) -> Result<(), ScxVoidError>;       // 执行安装
}
```

## 安装策略

### 工具链

| 组件 | macOS | Windows | 依赖 |
|------|-------|---------|------|
| fnm | `curl \| bash` | `winget` | 无 |
| Node.js | `fnm install --lts` | 同左 | fnm |
| pnpm | `npm install -g pnpm` | 同左 | Node.js |
| Bun | `curl \| bash` | PowerShell 脚本 | 无 |
| chsrc | GitHub releases 二进制下载 | 同左 | 无 |

### 编辑器

| 组件 | macOS | Windows |
|------|-------|---------|
| Cursor | 官网下载提示 | `winget install` |
| VS Code | `brew install --cask` | `winget install` |
| WebStorm | `brew install --cask` | `winget install` |
| Zed | `curl \| bash` | 不支持 |

## 安装后配置

- **fnm**: 自动追加 `eval "$(fnm env --use-on-cd)"` 到 `~/.zshrc` 或 `~/.bashrc`
- **chsrc**: 自动执行 `chsrc set node` 切换 npm 镜像源
- **ShellConfig**: 检测重复行，避免重复追加

## 扩展指南

新增安装组件：

1. 在 `src/services/setup/` 下创建新文件实现 `Installer` trait
2. 在 `SetupService::frontend()` 中注册新实例
3. 如仅特定平台可用，用 `#[cfg(target_os)]` 包裹注册
