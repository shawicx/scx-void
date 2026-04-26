/// 项目类型枚举（可扩展）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectType {
    NodeTsCli,
    Vue3,
    React,
    NestJs,
    NextJs,
}

#[allow(dead_code)]
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
            ProjectType::Vue3 => "vue3".to_string(),
            ProjectType::React => "react".to_string(),
            ProjectType::NestJs => "nestjs".to_string(),
            ProjectType::NextJs => "nextjs".to_string(),
        }
    }
}

/// GitHub 模板配置
#[derive(Debug, Clone)]
pub struct GitTemplate {
    /// 模板唯一标识符（用于命令行参数）
    pub id: String,

    /// 显示名称（用于交互式菜单）
    pub display_name: String,

    /// 模板描述
    #[allow(dead_code)]
    pub description: String,

    /// GitHub 仓库标识（owner/repo 格式）
    pub repository_url: String,

    /// 默认分支
    pub default_branch: String,

    /// 模板在仓库中的相对路径（空字符串表示根目录）
    pub template_path: String,

    /// 项目类型
    #[allow(dead_code)]
    pub project_type: ProjectType,

    /// 是否为用户自定义模板
    #[allow(dead_code)]
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
            project_type: ProjectType::NodeTsCli,
            is_custom: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_type_display_name() {
        assert_eq!(ProjectType::NodeTsCli.display_name(), "Node TypeScript CLI");
        assert_eq!(ProjectType::Vue3.display_name(), "Vue 3");
        assert_eq!(ProjectType::React.display_name(), "React");
    }

    #[test]
    fn test_project_type_identifier() {
        assert_eq!(ProjectType::NodeTsCli.identifier(), "node-ts-cli");
        assert_eq!(ProjectType::Vue3.identifier(), "vue3");
    }

    #[test]
    fn test_git_template_predefined() {
        let template = GitTemplate::predefined(
            "test",
            "Test Template",
            "Test Description",
            "shawicx/test-template",
            ProjectType::NodeTsCli,
        );

        assert_eq!(template.id, "test");
        assert_eq!(template.display_name, "Test Template");
        assert_eq!(template.repository_url, "shawicx/test-template");
        assert_eq!(template.default_branch, "main");
        assert!(!template.is_custom);
    }

    #[test]
    fn test_git_template_custom() {
        let template = GitTemplate::custom("user/repo", Some("develop"), Some("/src"));

        assert_eq!(template.id, "custom");
        assert_eq!(template.default_branch, "develop");
        assert_eq!(template.template_path, "/src");
        assert!(template.is_custom);
    }
}
