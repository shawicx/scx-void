# Git 模板下载功能设计文档

## 核心目标

实现一个可扩展的 Git 模板下载系统，支持交互式和命令式两种使用方式，支持以下项目类型：
- Node TypeScript CLI 项目
- Vue 3 项目
- React 项目
- NestJS 项目
- NextJS 项目

---

## 设计约束

1. **双模式支持**：必须同时支持交互式（CLI 引导）和命令式（参数直接指定）
2. **高度可扩展**：添加新模板类型不应修改核心逻辑，只需配置
3. **向后兼容**：保持现有本地模板功能不变

---

## 功能规范

### 1. 命令接口设计

#### 1.1 交互式模式
```bash
cargo run -- project init
```

**交互流程**：
```
步骤 1: 选择模板源
? 选择模板来源:
  > 本地模板
    Git 仓库模板

步骤 2 (选择 Git 后): 选择模板
? 选择 Git 模板:
  > node-ts-cli      Node TypeScript CLI 工具
    vue3-standard    Vue 3 标准项目
    react-modern     React 现代化项目
    nestjs-rest      NestJS REST API
    nextjs-app       NextJS App Router
    自定义仓库 URL

步骤 3 (选择自定义 URL 后): 输入仓库信息
? Git 仓库 URL: https://github.com/user/template.git
? 分支 (可选, 默认 main):
? 模板路径 (可选, 默认根目录):

步骤 4: 输入项目信息
? 项目名称: my-project

步骤 5: 确认并下载
✓ 正在克隆模板仓库...
✓ 正在生成项目结构...
✓ 项目创建成功!
```

#### 1.2 命令式模式
```bash
# 使用预定义模板
cargo run -- project init --source=git --template=node-ts-cli --name=my-project

# 使用自定义 Git 仓库
cargo run -- project init --source=git --url=https://github.com/user/template.git --name=my-project

# 指定分支
cargo run -- project init --source=git --template=node-ts-cli --branch=develop --name=my-project

# 指定模板子目录
cargo run -- project init --source=git --template=node-ts-cli --template-path=/base --name=my-project

# 完整参数示例
cargo run -- project init \
  --source=git \
  --url=https://github.com/user/template.git \
  --branch=v1.0.0 \
  --template-path=/templates/base \
  --name=my-project
```

#### 1.3 混合模式（部分交互）
```bash
# 指定 Git 源，其他交互式选择
cargo run -- project init --source=git

# 指定模板，交互式输入项目名
cargo run -- project init --template=node-ts-cli
```

---

## 数据结构设计

### 2.1 核心数据结构

**文件位置**: `src/services/project/git/types.rs`

```rust
/// 项目类型枚举（可扩展）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectType {
    NodeTsCli,
    Vue3,
    React,
    NestJs,
    NextJs,
    // 未来扩展:
    // PythonFastApi,
    // GoGin,
    // RustActix,
}

impl ProjectType {
    /// 获取项目类型的显示名称
    pub fn display_name(&self) -> String {
        match self {
            ProjectType::NodeTsCli => "Node TypeScript CLI".to_string(),
            ProjectType::Vue3 => "Vue 3".to_string(),
            ProjectType::React => "React".to_string(),
            ProjectType::NestJs => "NestJS".to_string(),
            ProjectType::NextJs => "NextJS".to_string(),
        }
    }

    /// 获取项目类型的唯一标识符
    pub fn identifier(&self) -> String {
        match self {
            ProjectType::NodeTsCli => "node-ts-cli".to_string(),
            ProjectType::Vue3 => "vue3-standard".to_string(),
            ProjectType::React => "react-modern".to_string(),
            ProjectType::NestJs => "nestjs-rest".to_string(),
            ProjectType::NextJs => "nextjs-app".to_string(),
        }
    }
}

/// Git 模板配置
#[derive(Debug, Clone)]
pub struct GitTemplate {
    /// 模板唯一标识符（用于命令行参数）
    pub id: String,

    /// 显示名称（用于交互式菜单）
    pub display_name: String,

    /// 模板描述
    pub description: String,

    /// Git 仓库 URL
    pub repository_url: String,

    /// 默认分支
    pub default_branch: String,

    /// 模板在仓库中的相对路径（空字符串表示根目录）
    pub template_path: String,

    /// 项目类型
    pub project_type: ProjectType,

    /// 是否为用户自定义模板
    pub is_custom: bool,
}

impl GitTemplate {
    /// 创建预定义模板
    pub fn predefined(
        id: &str,
        display_name: &str,
        description: &str,
        repository_url: &str,
        project_type: ProjectType,
    ) -> Self {
        Self {
            id: id.to_string(),
            display_name: display_name.to_string(),
            description: description.to_string(),
            repository_url: repository_url.to_string(),
            default_branch: "main".to_string(),
            template_path: String::new(),
            project_type,
            is_custom: false,
        }
    }

    /// 创建自定义模板
    pub fn custom(
        repository_url: &str,
        branch: Option<&str>,
        template_path: Option<&str>,
    ) -> Self {
        Self {
            id: "custom".to_string(),
            display_name: "自定义仓库".to_string(),
            description: format!("来自 {}", repository_url),
            repository_url: repository_url.to_string(),
            default_branch: branch.unwrap_or("main").to_string(),
            template_path: template_path.unwrap_or("").to_string(),
            project_type: ProjectType::NodeTsCli, // 默认类型
            is_custom: true,
        }
    }
}

/// Git 克隆选项
#[derive(Debug, Clone)]
pub struct CloneOptions {
    pub repository_url: String,
    pub branch: Option<String>,
    pub target_dir: String,
    pub depth: Option<u32>,  // 浅克隆深度，提高速度
    pub sparse_checkout: bool, // 是否使用稀疏检出
}

impl CloneOptions {
    pub fn new(repository_url: String, target_dir: String) -> Self {
        Self {
            repository_url,
            branch: None,
            target_dir,
            depth: Some(1), // 默认浅克隆，只克隆最新提交
            sparse_checkout: false,
        }
    }

    pub fn with_branch(mut self, branch: String) -> Self {
        self.branch = Some(branch);
        self
    }

    pub fn with_depth(mut self, depth: u32) -> Self {
        self.depth = Some(depth);
        self
    }
}

/// 模板源枚举
#[derive(Debug, Clone)]
pub enum TemplateSource {
    Local(String),  // 本地模板名称
    Git(GitTemplate), // Git 模板配置
}
```

### 2.2 模板注册表设计

**文件位置**: `src/services/project/git/registry.rs`

```rust
use std::sync::LazyLock;
use std::collections::HashMap;

/// 模板注册表（全局单例）
static TEMPLATE_REGISTRY: LazyLock<HashMap<String, GitTemplate>> = LazyLock::new(|| {
    let mut registry = HashMap::new();

    // 注册预定义模板
    registry.insert(
        "node-ts-cli".to_string(),
        GitTemplate::predefined(
            "node-ts-cli",
            "Node TypeScript CLI",
            "基于 Node.js + TypeScript 的 CLI 工具项目模板",
            "https://github.com/your-org/node-ts-cli-template.git",
            ProjectType::NodeTsCli,
        )
    );

    registry.insert(
        "vue3-standard".to_string(),
        GitTemplate::predefined(
            "vue3-standard",
            "Vue 3 标准",
            "Vue 3 + TypeScript + Vite 标准项目模板",
            "https://github.com/your-org/vue3-template.git",
            ProjectType::Vue3,
        )
    );

    registry.insert(
        "react-modern".to_string(),
        GitTemplate::predefined(
            "react-modern",
            "React 现代化",
            "React 18 + TypeScript + Vite 现代化项目模板",
            "https://github.com/your-org/react-template.git",
            ProjectType::React,
        )
    );

    registry.insert(
        "nestjs-rest".to_string(),
        GitTemplate::predefined(
            "nestjs-rest",
            "NestJS REST API",
            "NestJS RESTful API 项目模板",
            "https://github.com/your-org/nestjs-template.git",
            ProjectType::NestJs,
        )
    );

    registry.insert(
        "nextjs-app".to_string(),
        GitTemplate::predefined(
            "nextjs-app",
            "NextJS App Router",
            "NextJS 14 + App Router + TypeScript 项目模板",
            "https://github.com/your-org/nextjs-template.git",
            ProjectType::NextJs,
        )
    );

    registry
});

/// 获取所有预定义模板
pub fn get_all_templates() -> Vec<GitTemplate> {
    TEMPLATE_REGISTRY.values().cloned().collect()
}

/// 根据 ID 获取模板
pub fn get_template_by_id(id: &str) -> Option<GitTemplate> {
    TEMPLATE_REGISTRY.get(id).cloned()
}

/// 检查模板 ID 是否存在
pub fn template_exists(id: &str) -> bool {
    TEMPLATE_REGISTRY.contains_key(id)
}

/// 添加自定义模板到运行时注册表（可选功能）
pub fn register_custom_template(template: GitTemplate) {
    // 注意：这里使用 unsafe 或内部可变性来实现运行时注册
    // 或者使用 RwLock<HashMap> 替代 LazyLock
}
```

---

## 模块结构设计

### 3.1 新增文件结构

```
src/
├── services/
│   └── project/
│       ├── git/
│       │   ├── mod.rs              # Git 模块入口，导出公共接口
│       │   ├── types.rs            # 数据结构定义（GitTemplate, CloneOptions 等）
│       │   ├── registry.rs         # 模板注册表（预定义模板列表）
│       │   ├── clone.rs            # Git 克隆实现
│       │   ├── downloader.rs       # 模板下载器（协调克隆和文件复制）
│       │   └── validator.rs        # URL 和参数验证
│       ├── generator.rs            # 修改：集成 Git 模板生成
│       ├── project_service.rs      # 修改：添加 Git 模板创建逻辑
│       └── mod.rs
├── cli/
│   ├── project.rs                  # 修改：添加 Git 源 CLI 参数和交互
│   └── mod.rs
├── utils/
│   ├── git.rs                      # 新增：Git 命令工具函数
│   └── mod.rs
└── errors.rs                       # 修改：添加 Git 相关错误类型
```

### 3.2 模块职责划分

| 模块 | 职责 |
|------|------|
| `git/types.rs` | 定义所有数据结构，不包含业务逻辑 |
| `git/registry.rs` | 管理模板配置，提供模板查询接口 |
| `git/validator.rs` | 验证 URL、分支名等输入参数 |
| `git/clone.rs` | 执行 Git 克隆操作，使用 `duct` 调用系统 git |
| `git/downloader.rs` | 协调克隆、文件提取、清理临时目录 |
| `utils/git.rs` | 通用 Git 工具函数（可被其他模块复用） |
| `generator.rs` | 项目生成逻辑，处理模板变量替换 |
| `project_service.rs` | 业务逻辑编排，协调各个模块 |
| `cli/project.rs` | 用户交互，参数解析和菜单展示 |

---

## 核心功能实现规范

### 4.1 Git 克隆模块

**文件**: `src/services/project/git/clone.rs`

```rust
use crate::errors::ScxVoidError;
use crate::services::project::git::types::CloneOptions;
use duct::cmd;

/// 克隆 Git 仓库
///
/// # 参数
/// * `options` - 克隆选项
///
/// # 返回
/// * `Ok(())` - 克隆成功
/// * `Err(ScxVoidError)` - 克隆失败
///
/// # 示例
/// ```
/// let options = CloneOptions::new(
///     "https://github.com/user/repo.git".to_string(),
///     "/tmp/repo".to_string()
/// ).with_branch("main".to_string());
///
/// clone_repository(options)?;
/// ```
pub async fn clone_repository(options: CloneOptions) -> Result<(), ScxVoidError> {
    // 1. 验证目标目录不存在
    // 2. 构建 git clone 命令
    // 3. 执行克隆（支持浅克隆和分支指定）
    // 4. 验证克隆成功
    // 5. 返回结果
}

/// 克隆仓库的特定子目录（使用稀疏检出）
pub async fn clone_sparse_checkout(
    options: CloneOptions,
    sparse_path: &str,
) -> Result<(), ScxVoidError> {
    // 实现稀疏检出逻辑
}
```

### 4.2 模板下载器

**文件**: `src/services/project/git/downloader.rs`

```rust
use crate::errors::ScxVoidError;
use crate::services::project::git::types::GitTemplate;
use tempfile::TempDir;

/// 下载 Git 模板到临时目录
///
/// # 流程
/// 1. 创建临时目录
/// 2. 克隆 Git 仓库
/// 3. 切换到指定分支（如果提供）
/// 4. 返回临时目录路径（用于后续文件复制）
///
/// # 注意
/// 调用者负责在完成后清理临时目录，或使用 TempDir 自动清理
pub async fn download_template_to_temp(
    template: &GitTemplate,
    branch: Option<&str>,
) -> Result<TempDir, ScxVoidError> {
    // 1. 创建临时目录
    // 2. 构建克隆选项
    // 3. 执行克隆
    // 4. 返回 TempDir（自动管理生命周期）
}

/// 从临时目录提取模板文件到项目目录
pub fn extract_template_files(
    temp_dir: &TempDir,
    template: &GitTemplate,
    project_name: &str,
) -> Result<(), ScxVoidError> {
    // 1. 确定模板源路径（temp_dir + template.template_path）
    // 2. 复制文件到项目目录
    // 3. 过滤 .git 目录
    // 4. 根据项目类型执行额外配置
}
```

### 4.3 项目服务集成

**文件**: `src/services/project/project_service.rs`

```rust
/// 使用 Git 模板创建项目
pub async fn create_project_from_git(
    project_name: &str,
    template: &GitTemplate,
    branch: Option<&str>,
) -> Result<(), ScxVoidError> {
    // 1. 验证项目名称
    // 2. 检查项目目录是否已存在
    // 3. 下载模板到临时目录
    // 4. 提取模板文件到项目目录
    // 5. 根据项目类型执行额外配置（调用现有 create_xxx_files）
    // 6. 临时目录自动清理（TempDir Drop）
}

/// 创建项目（统一入口，支持本地和 Git 模板）
pub async fn create_project(
    project_name: &str,
    template_source: TemplateSource,
) -> Result<(), ScxVoidError> {
    match template_source {
        TemplateSource::Local(template_name) => {
            // 现有逻辑：调用 create_project_with_type_index
        }
        TemplateSource::Git(template) => {
            create_project_from_git(project_name, &template, None).await?;
        }
    }
}
```

---

## CLI 集成规范

### 5.1 CLI 参数定义

**文件**: `src/cli/project.rs`

```rust
use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    /// 初始化一个新项目
    Init {
        /// 模板源 (local 或 git)
        #[arg(short, long)]
        source: Option<String>,

        /// Git 模板 ID（预定义模板的唯一标识）
        #[arg(short, long)]
        template: Option<String>,

        /// 自定义 Git 仓库 URL
        #[arg(short, long)]
        url: Option<String>,

        /// Git 分支或标签
        #[arg(short, long)]
        branch: Option<String>,

        /// 模板在仓库中的相对路径
        #[arg(long)]
        template_path: Option<String>,

        /// 项目名称
        #[arg(short, long)]
        name: Option<String>,
    },

    // ... 其他命令
}
```

### 5.2 交互式流程实现

```rust
async fn init_project() {
    // 步骤 1: 选择模板源
    let source = select_template_source().await;

    match source {
        TemplateSource::Local(template_name) => {
            // 现有逻辑
        }
        TemplateSource::Git(template) => {
            // 步骤 2: 获取项目名称
            let project_name = get_project_name().await;

            // 步骤 3: 创建项目
            match create_project_from_git(&project_name, &template, None).await {
                Ok(_) => println!("✓ 项目创建成功!"),
                Err(e) => eprintln!("✗ 创建项目失败: {}", e),
            }
        }
    }
}

/// 交互式选择模板源
async fn select_template_source() -> TemplateSource {
    let sources = ["本地模板", "Git 仓库模板"];

    let selection = Select::new()
        .with_prompt("选择模板来源")
        .items(&sources)
        .default(0)
        .interact()
        .unwrap();

    if selection == 1 {
        select_git_template().await
    } else {
        // 现有本地模板选择逻辑
        TemplateSource::Local("node_ts".to_string())
    }
}

/// 交互式选择 Git 模板
async fn select_git_template() -> TemplateSource {
    let templates = get_all_templates();

    // 构建显示列表
    let items: Vec<String> = templates
        .iter()
        .map(|t| format!("{} - {}", t.display_name, t.description))
        .collect();

    // 添加"自定义仓库"选项
    let mut display_items = items.clone();
    display_items.push("自定义仓库 URL".to_string());

    let selection = Select::new()
        .with_prompt("选择 Git 模板")
        .items(&display_items)
        .default(0)
        .interact()
        .unwrap();

    if selection == templates.len() {
        // 自定义 URL
        input_custom_template().await
    } else {
        TemplateSource::Git(templates[selection].clone())
    }
}

/// 输入自定义模板信息
async fn input_custom_template() -> TemplateSource {
    let url: String = Input::new()
        .with_prompt("Git 仓库 URL")
        .interact()
        .unwrap();

    let branch: String = Input::new()
        .with_prompt("分支 (可选, 默认 main)")
        .allow_empty(true)
        .with_initial_value("main")
        .interact()
        .unwrap();

    let template_path: String = Input::new()
        .with_prompt("模板路径 (可选, 默认根目录)")
        .allow_empty(true)
        .interact()
        .unwrap();

    let template = GitTemplate::custom(
        &url,
        if branch.is_empty() { None } else { Some(&branch) },
        if template_path.is_empty() { None } else { Some(&template_path) },
    );

    TemplateSource::Git(template)
}
```

---

## 扩展性设计

### 6.1 添加新模板的步骤

**步骤 1**: 在 `registry.rs` 中注册新模板

```rust
registry.insert(
    "python-fastapi".to_string(),
    GitTemplate::predefined(
        "python-fastapi",
        "Python FastAPI",
        "FastAPI + Python 项目模板",
        "https://github.com/your-org/fastapi-template.git",
        ProjectType::PythonFastApi, // 新增枚举值
    )
);
```

**步骤 2**: 在 `ProjectType` 枚举中添加新类型

```rust
pub enum ProjectType {
    // ... 现有类型
    PythonFastApi,  // 新增
}
```

**步骤 3**: 在 `project_service.rs` 中添加对应配置函数（如果需要特殊处理）

```rust
async fn create_python_fastapi_files(project_name: &str) -> Result<(), ScxVoidError> {
    // 项目特定的配置逻辑
}
```

**完成** - 无需修改核心逻辑！

### 6.2 配置文件扩展（未来功能）

**文件**: `~/.scx-void/templates.toml`

```toml
# 预定义模板（覆盖内置模板）
[templates.node-ts-cli]
repository_url = "https://github.com/custom/node-ts-template.git"
default_branch = "main"
description = "自定义 Node TS CLI 模板"

# 完全自定义模板
[templates.my-company-react]
repository_url = "https://github.com/my-company/react-template.git"
default_branch = "production"
template_path = "/packages/base"
description = "公司内部 React 模板"

# 用户自定义模板可被自动加载和注册
```

---

## 错误处理规范

### 7.1 新增错误类型

**文件**: `src/errors.rs`

```rust
pub enum ScxVoidError {
    // ... 现有错误类型

    /// Git 克隆失败
    #[error("Git 克隆失败: {0}")]
    GitCloneError(String),

    /// 无效的 Git URL
    #[error("无效的 Git URL: {0}")]
    InvalidGitUrl(String),

    /// Git 分支不存在
    #[error("Git 分支不存在: {0}")]
    GitBranchNotFound(String),

    /// 模板下载失败
    #[error("模板下载失败: {0}")]
    TemplateDownloadFailed(String),

    /// 模板 ID 不存在
    #[error("模板 '{0}' 不存在")]
    TemplateNotFound(String),

    /// Git 命令未找到
    #[error("系统未安装 Git 命令")]
    GitNotInstalled,
}
```

### 7.2 错误处理模式

```rust
// 在 clone_repository 中
pub async fn clone_repository(options: CloneOptions) -> Result<(), ScxVoidError> {
    // 检查 git 是否安装
    if !is_git_installed() {
        return Err(ScxVoidError::GitNotInstalled);
    }

    // 验证 URL
    if !validate_git_url(&options.repository_url) {
        return Err(ScxVoidError::InvalidGitUrl(options.repository_url));
    }

    // 执行克隆并捕获错误
    cmd!("git", "clone", ...)
        .run()
        .map_err(|e| ScxVoidError::GitCloneError(e.to_string()))?;

    Ok(())
}
```

---

## 测试策略

### 8.1 单元测试

**文件**: `src/services/project/git/clone_test.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_git_url_valid_https() {
        assert!(validate_git_url("https://github.com/user/repo.git"));
    }

    #[test]
    fn test_validate_git_url_invalid() {
        assert!(!validate_git_url("not-a-url"));
    }

    #[test]
    fn test_template_registry_contains_all_templates() {
        let templates = get_all_templates();
        assert_eq!(templates.len(), 5); // 5 个预定义模板
    }
}
```

### 8.2 集成测试

**文件**: `tests/git_template_test.rs`

```rust
#[tokio::test]
async fn test_create_project_from_git_template() {
    // 使用测试仓库创建项目
    let template = GitTemplate::predefined(
        "test-template",
        "Test Template",
        "Test description",
        "https://github.com/test/test-template.git",
        ProjectType::NodeTsCli,
    );

    let result = create_project_from_git("test_project", &template, None).await;
    assert!(result.is_ok());

    // 验证项目目录创建成功
    assert!(Path::new("test_project").exists());

    // 清理
    std::fs::remove_dir_all("test_project").unwrap();
}
```

---

## 实现顺序建议

### Phase 1: 基础设施（1-2天）
- [ ] 创建 `src/services/project/git/` 模块目录结构
- [ ] 实现 `src/services/project/git/types.rs` - 定义所有数据结构（ProjectType、GitTemplate、CloneOptions、TemplateSource）
- [ ] 实现 `src/services/project/git/registry.rs` - 模板注册表和预定义模板配置
- [ ] 实现 `src/services/project/git/validator.rs` - URL 和参数验证逻辑
- [ ] 实现 `src/utils/git.rs` - Git 命令工具函数（is_git_installed、validate_git_url 等）
- [ ] 修改 `src/errors.rs` - 添加 Git 相关错误类型（GitCloneError、InvalidGitUrl、GitNotInstalled 等）

### Phase 2: 核心功能（2-3天）
- [ ] 实现 `src/services/project/git/clone.rs` - Git 克隆功能（clone_repository、clone_sparse_checkout）
- [ ] 实现 `src/services/project/git/downloader.rs` - 模板下载器（download_template_to_temp、extract_template_files）
- [ ] 修改 `src/services/project/generator.rs` - 集成 Git 模板生成逻辑
- [ ] 修改 `src/services/project/project_service.rs` - 添加 create_project_from_git 和统一入口函数
- [ ] 实现 `src/services/project/git/mod.rs` - 模块入口，导出公共接口

### Phase 3: CLI 集成（1-2天）
- [ ] 修改 `src/cli/project.rs` - 添加 Git 源相关 CLI 参数（--source、--template、--url、--branch、--template_path）
- [ ] 实现交互式模板源选择流程（select_template_source）
- [ ] 实现 Git 模板选择交互（select_git_template）
- [ ] 实现自定义模板输入流程（input_custom_template）
- [ ] 实现命令式参数解析和路由逻辑
- [ ] 添加用户友好的提示信息和错误反馈

### Phase 4: 测试和优化（1-2天）
- [ ] 编写 `src/services/project/git/clone_test.rs` - Git 克隆功能单元测试
- [ ] 编写 `src/services/project/git/validator_test.rs` - 验证逻辑单元测试
- [ ] 编写 `tests/git_template_test.rs` - 集成测试（完整项目创建流程）
- [ ] 优化错误处理和错误消息
- [ ] 添加下载进度提示
- [ ] 实现 TempDir 自动清理验证

### Phase 5: 文档和发布（1天）
- [ ] 更新项目 README - 添加 Git 模板功能说明
- [ ] 编写使用文档 - 交互式和命令式使用示例
- [ ] 准备示例项目模板（可选）
- [ ] 验证向后兼容性（本地模板功能正常）
- [ ] 运行 `cargo clippy` 和 `cargo fmt` 检查
- [ ] 最终集成测试和发布

**总计**: 6-10 天

---

### 快速开始检查清单

开始实现前，请确认：

- [ ] 已阅读完整设计文档
- [ ] 已安装 Rust 开发环境
- [ ] 系统已安装 Git 命令
- [ ] 理解现有项目架构
- [ ] 准备好测试用 Git 仓库（可选）

---

## 依赖清单

```toml
[dependencies]
# 现有依赖
clap = { version = "4.x", features = ["derive"] }
dialoguer = "0.x"
duct = "0.x"
tokio = { version = "1.x", features = ["full"] }

# 新增依赖
tempfile = "3.x"        # 临时目录管理
url = "2.x"             # URL 解析和验证
```

---

## AI 实现检查清单

在实现过程中，请确保：

- [ ] 所有公共函数都有完整的文档注释（`///`）
- [ ] 所有公共结构体都实现了 `Debug` 和 `Clone` trait
- [ ] 错误处理使用 `Result<T, ScxVoidError>` 模式
- [ ] 交互式输入使用 `dialoguer` 库
- [ ] Git 命令使用 `duct` 库执行
- [ ] 临时文件使用 `tempfile` 库自动清理
- [ ] 添加了相应的单元测试
- [ ] 向后兼容本地模板功能
- [ ] 代码通过 `cargo clippy` 检查
- [ ] 代码格式化符合 `cargo fmt` 标准
