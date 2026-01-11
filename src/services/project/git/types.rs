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
    pub fn custom(repository_url: &str, branch: Option<&str>, template_path: Option<&str>) -> Self {
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
    pub depth: Option<u32>,    // 浅克隆深度，提高速度
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

    pub fn with_sparse_checkout(mut self, sparse: bool) -> Self {
        self.sparse_checkout = sparse;
        self
    }
}

/// 模板源枚举
#[derive(Debug, Clone)]
pub enum TemplateSource {
    Local(String),    // 本地模板名称
    Git(GitTemplate), // Git 模板配置
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_type_display_name() {
        assert_eq!(ProjectType::NodeTsCli.display_name(), "Node TypeScript CLI");
        assert_eq!(ProjectType::Vue3.display_name(), "Vue 3");
    }

    #[test]
    fn test_project_type_identifier() {
        assert_eq!(ProjectType::NodeTsCli.identifier(), "node-ts-cli");
        assert_eq!(ProjectType::Vue3.identifier(), "vue3-standard");
    }

    #[test]
    fn test_git_template_predefined() {
        let template = GitTemplate::predefined(
            "test-id",
            "Test Template",
            "Test Description",
            "https://github.com/test/repo.git",
            ProjectType::React,
        );

        assert_eq!(template.id, "test-id");
        assert_eq!(template.display_name, "Test Template");
        assert_eq!(template.repository_url, "https://github.com/test/repo.git");
        assert_eq!(template.default_branch, "main");
        assert_eq!(template.is_custom, false);
    }

    #[test]
    fn test_git_template_custom() {
        let template = GitTemplate::custom(
            "https://github.com/custom/repo.git",
            Some("develop"),
            Some("/templates/base"),
        );

        assert_eq!(template.id, "custom");
        assert_eq!(template.default_branch, "develop");
        assert_eq!(template.template_path, "/templates/base");
        assert_eq!(template.is_custom, true);
    }

    #[test]
    fn test_clone_options_builder() {
        let options = CloneOptions::new(
            "https://github.com/test/repo.git".to_string(),
            "/tmp/repo".to_string(),
        )
        .with_branch("main".to_string())
        .with_depth(1);

        assert_eq!(options.repository_url, "https://github.com/test/repo.git");
        assert_eq!(options.branch, Some("main".to_string()));
        assert_eq!(options.depth, Some(1));
        assert_eq!(options.sparse_checkout, false);
    }
}
