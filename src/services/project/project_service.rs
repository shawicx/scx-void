use crate::errors::ScxVoidError;
use crate::services::project::generator;
use crate::services::project::git::types::GitTemplate;
use crate::services::project::git::validator;
use std::path::Path;

/// 创建项目（唯一入口）
///
/// 从 GitHub 模板下载并生成项目
pub async fn create_project(
    project_name: &str,
    template: &GitTemplate,
    branch: Option<&str>,
) -> Result<(), ScxVoidError> {
    // 验证项目名
    if project_name.trim().is_empty() {
        return Err(ScxVoidError::InvalidProjectName(
            "项目名称不能为空".to_string(),
        ));
    }

    // 检查目录是否已存在
    if Path::new(project_name).exists() {
        return Err(ScxVoidError::ProjectAlreadyExists(project_name.to_string()));
    }

    // 验证模板
    validator::validate_git_template(template)?;

    // 生成项目
    generator::generate_from_github_template(project_name, template, branch).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::project::git::types::ProjectType;

    #[tokio::test]
    async fn test_create_project_with_valid_name() {
        let template = GitTemplate::predefined(
            "test",
            "Test",
            "Desc",
            "shawicx/test-template",
            ProjectType::NodeTsCli,
        );
        let result = create_project("", &template, None).await;
        assert!(result.is_err());
    }
}
