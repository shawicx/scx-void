# 项目服务

项目服务负责从 GitHub 模板仓库初始化项目和 AI 规则文件管理。

## 模块总览

```
src/services/project/
├── mod.rs                 # 模块导出（create_project）
├── project_service.rs     # 项目创建主逻辑（唯一入口）
├── generator.rs           # GitHub 模板生成器
├── ai_rule.rs             # AI 规则文件管理
└── git/                   # GitHub 模板系统
    ├── mod.rs
    ├── types.rs           # 数据类型定义（GitTemplate, ProjectType）
    ├── registry.rs        # 模板注册表
    ├── archive.rs         # GitHub zip 归档下载与解压
    ├── downloader.rs      # 下载编排器
    └── validator.rs       # 模板验证器
```

---

## 项目创建流程

**关联代码**：`src/services/project/project_service.rs`

### 入口函数

```rust
pub async fn create_project(
    project_name: &str,
    template: &GitTemplate,
    branch: Option<&str>,
) -> Result<(), ScxVoidError>
```

唯一的创建入口，从 GitHub 模板下载并生成项目。

### 创建流程

```
用户调用 CLI（--template / --url / 交互式选择）
            │
            ▼
    create_project(name, template, branch)
            │
            ├── 验证项目名
            ├── 检查目录不存在
            ├── validate_git_template()
            │
            ▼
    generate_from_github_template(name, template, branch)
            │
            ▼
    download_template_to_temp(template, branch)
            │
            ├── parse_github_url() → (owner, repo)
            ├── build_archive_url() → codeload zip URL
            ├── reqwest 流式下载（indicatif 进度条）
            ├── zip::ZipArchive 解压到临时目录
            │
            ▼
    extract_template_files(source_dir, project_name)
            │
            ▼
    项目创建完成
```

### 技术要点

- **无需安装 git** — 使用 GitHub codeload API 下载 zip 归档
- **流式下载** — `reqwest` + `futures_util::StreamExt` 流式写入，配合 `indicatif` 进度条
- **自动解压** — `zip` crate 解压到临时目录，然后 `copy_dir_all` 到目标

---

## GitHub 模板系统

**关联代码**：`src/services/project/git/`

### 核心类型

**关联代码**：`src/services/project/git/types.rs`

| 类型 | 说明 |
|------|------|
| `ProjectType` | 项目类型枚举（NodeTsCli / Vue3 / React / NestJs / NextJs） |
| `GitTemplate` | 模板配置（id、display_name、repository_url、default_branch、template_path） |

`GitTemplate` 有两种构造方式：
- `GitTemplate::predefined()` — 从注册表创建预定义模板
- `GitTemplate::custom()` — 用户指定的自定义仓库

### 预定义模板注册表

**关联代码**：`src/services/project/git/registry.rs`

| ID | 仓库 | 技术栈 |
|----|------|--------|
| `node-ts-cli` | `shawicx/scx-template-cli` | TypeScript + Node.js |
| `vue3-standard` | `shawicx/template-vue3-standard` | Vue 3 + TypeScript + Vite |
| `react-modern` | `shawicx/template-react-modern` | React 18 + TypeScript + Vite |
| `nestjs-rest` | `shawicx/template-nestjs-rest` | NestJS + TypeScript |
| `nextjs-app` | `shawicx/template-nextjs-app` | NextJS 14 + App Router + TypeScript |

注册表提供以下查询能力：
- `get_all_templates()` — 获取所有模板
- `get_template_by_id()` — 按 ID 查找
- `template_exists()` — 检查存在性
- `get_template_map()` — ID 到模板的映射

### 添加新模板

1. 在 `registry.rs` 的 `get_all_templates()` 中添加 `GitTemplate::predefined(...)` 条目
2. 确保 GitHub 仓库存在（如 `shawicx/template-xxx`）
3. 如需新的 `ProjectType`，在 `types.rs` 中扩展枚举

---

## 归档下载模块

**关联代码**：`src/services/project/git/archive.rs`

核心函数 `download_and_extract_archive(owner, repo, branch, target_dir)`：

1. 构建 URL：`https://codeload.github.com/{owner}/{repo}/zip/refs/heads/{branch}`
2. `reqwest` 流式下载到临时 `.zip` 文件
3. `zip::ZipArchive` 解压到 `target_dir`
4. 返回解压后的内容路径（`{repo}-{branch}/`）

---

## AI 规则文件管理

**关联代码**：`src/services/project/ai_rule.rs`

`AiRuleService` 负责管理 `AGENTS.md` 文件的生成和更新。

### 功能

| 方法 | 说明 |
|------|------|
| `new()` | 创建服务实例 |
| `manage_rule_file()` | 生成/更新规则文件 |
| `get_template_content()` | 获取模板内容 |
| `validate_existing_file()` | 检查文件是否已存在 |
| `backup_existing_file()` | 创建 `.bak` 备份 |

### 模板

**关联代码**：`assets/templates/ai_rule/`

| 模板 | 说明 |
|------|------|
| `basic.md` | 简单的 AI 代理规则 |
| `advanced.md` | 完整的多项目规则，涵盖依赖管理、项目结构、兼容性要求等 |

### 行为流程

```
选择模板类型
    │
    ▼
检查 AGENTS.md 是否存在
    │
    ├── 不存在 → 直接生成
    │
    └── 存在 → 检查 --force 标志
                │
                ├── 无 --force → 返回 AiRuleFileExists 错误
                │
                └── 有 --force → 备份为 .bak → 生成新文件
```

---

## 相关文档

- [架构概览](../../Overview/Architecture.md)
- [CLI 命令参考](../../cli/CLI-Reference.md)
- [错误处理](../../Overview/Error-Handling.md)

---

> 最后更新：2026-04-26
