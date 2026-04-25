# 项目服务

项目服务负责项目初始化、模板生成、Git 模板管理和 AI 规则文件管理。

## 模块总览

```
src/services/project/
├── mod.rs                 # 模块导出
├── project_service.rs     # 项目创建主逻辑
├── project.rs             # Project 服务结构体
├── generator.rs           # 模板生成器
├── ai_rule.rs             # AI 规则文件管理
├── templates/             # 内置模板定义
│   ├── mod.rs
│   ├── node_ts.rs
│   ├── react.rs
│   └── vue.rs
├── installers/            # 项目安装器
│   ├── mod.rs
│   ├── node_ts.rs
│   ├── react.rs
│   ├── vue.rs
│   ├── nestjs.rs
│   ├── nextjs.rs
│   └── tailwind.rs
└── git/                   # Git 模板系统
    ├── mod.rs
    ├── types.rs           # 数据类型定义
    ├── registry.rs        # 模板注册表
    ├── downloader.rs      # 模板下载器
    ├── validator.rs       # 模板验证器
    └── clone.rs           # Git 克隆逻辑
```

---

## 项目创建流程

**关联代码**：`src/services/project/project_service.rs`

### 入口函数

```
create_project(name, type_index)      → 本地模板创建
create_project_with_source(...)        → 指定来源创建
create_project_from_git(...)           → Git 模板创建
```

### 支持的项目类型

| 索引 | 类型 | 安装器 |
|------|------|--------|
| 0 | Node TypeScript | `installers/node_ts.rs` |
| 1 | React | `installers/react.rs` |
| 2 | Vue | `installers/vue.rs` |
| 3 | NestJS | `installers/nestjs.rs` |
| 4 | NextJS | `installers/nextjs.rs` |

### 创建流程

```
用户调用 CLI → 选择模板来源
                │
    ┌───────────┼───────────┐
    ▼           ▼           ▼
  local       git        interactive
    │           │           │
    ▼           ▼           ▼
 内置模板    Git 注册表   dialoguer 交互
    │           │           │
    └───────────┼───────────┘
                ▼
        generator 生成文件
                ▼
        installer 执行安装
```

---

## 模板生成器

**关联代码**：`src/services/project/generator.rs`

模板生成器负责根据项目类型创建文件结构：

- `generate_from_template()` — 从内置模板生成
- `generate_from_git_template()` — 从 Git 模板生成

生成内容包括：
- `package.json` — 项目配置
- `tsconfig.json` — TypeScript 配置
- 其他项目特定文件

---

## 安装器（Installers）

**关联代码**：`src/services/project/installers/`

每种项目类型有对应的安装器模块，负责：
1. 创建项目文件结构
2. 生成配置文件
3. 初始化依赖

### 扩展新项目类型

添加新安装器的步骤：

1. 在 `installers/` 下创建新文件（如 `svelte.rs`）
2. 在 `installers/mod.rs` 中注册模块
3. 在 `project_service.rs` 中添加类型索引映射

无需修改现有代码，符合**插件式扩展**原则。

---

## Git 模板系统

**关联代码**：`src/services/project/git/`

### 核心类型

**关联代码**：`src/services/project/git/types.rs`

| 类型 | 说明 |
|------|------|
| `ProjectType` | 项目类型枚举 |
| `GitTemplate` | Git 模板配置（URL、描述、默认分支） |
| `CloneOptions` | 克隆选项（支持 sparse checkout） |
| `TemplateSource` | 模板来源：Local / Git / Custom |

### 预定义模板注册表

**关联代码**：`src/services/project/git/registry.rs`

| ID | 描述 | 技术栈 |
|----|------|--------|
| `node-ts-cli` | Node TypeScript CLI | TypeScript + Node.js |
| `vue3-standard` | Vue 3 标准 | Vue 3 + TypeScript + Vite |
| `react-modern` | React 现代模板 | React 18 + TypeScript + Vite |
| `nestjs-rest` | NestJS REST API | NestJS + TypeScript |
| `nextjs-app` | NextJS App Router | NextJS 14 + App Router + TypeScript |

注册表提供以下查询能力：
- `get_all_templates()` — 获取所有模板
- `get_template_by_id()` — 按 ID 查找
- `template_exists()` — 检查存在性
- `get_template_map()` — ID 到模板的映射

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

> 最后更新：2026-04-25
