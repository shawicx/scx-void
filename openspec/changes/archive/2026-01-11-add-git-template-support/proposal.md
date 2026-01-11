# 变更：添加 Git 模板支持

## Why

当前 scx-void 项目仅支持本地模板初始化，用户无法从 Git 仓库直接获取项目模板。添加 Git 模板支持将提供更灵活的项目初始化方式，支持用户自定义模板和团队协作，同时保持向后兼容现有本地模板功能。

## What Changes

- 添加 Git 模板下载系统，支持从 Git 仓库克隆项目模板
- 扩展 CLI 参数，支持 `--source`、`--template`、`--url`、`--branch`、`--template-path` 选项
- 创建模板注册表系统，支持预定义模板和自定义 Git 仓库
- 实现双模式使用：交互式（CLI 引导）和命令式（参数直接指定）
- 添加稀疏检出支持，可只克隆仓库的特定子目录
- 扩展错误类型，添加 Git 相关错误处理

**新增功能**：
- Git 仓库克隆（使用 `duct` 调用系统 git）
- 模板验证（URL、分支名验证）
- 临时文件管理（使用 `tempfile` 自动清理）
- Git 命令工具函数（可复用的 Git 操作）

### 受影响的代码
- `src/cli/project.rs` - 扩展 CLI 参数，添加 Git 模板交互流程
- `src/services/project/project_service.rs` - 添加 Git 模板创建逻辑
- `src/services/project/generator.rs` - 集成 Git 模板生成
- `src/errors.rs` - 添加 Git 相关错误类型（`GitCloneError`、`InvalidGitUrl`、`GitNotInstalled` 等）

### 新增模块
- `src/services/project/git/` - Git 模板核心功能
  - `mod.rs` - 模块入口
  - `types.rs` - 数据结构定义（`GitTemplate`、`CloneOptions`、`ProjectType` 等）
  - `registry.rs` - 模板注册表和预定义模板配置
  - `validator.rs` - URL 和参数验证
  - `clone.rs` - Git 克隆实现
  - `downloader.rs` - 模板下载器和文件提取
- `src/utils/git.rs` - 通用 Git 工具函数

### 向后兼容性
- 现有本地模板功能保持不变
- 现有 CLI 命令行为兼容（不指定 `--source` 时默认为本地模板）

### 依赖变更
- 新增：`tempfile = "3.x"` - 临时目录管理
- 新增：`url = "2.x"` - URL 解析和验证
- 已有：`clap`、`dialoguer`、`duct`、`tokio`（现有依赖）
