use crate::errors::ScxVoidError;
use crate::services::project::git::registry;
use crate::services::project::git::types::GitTemplate;
use crate::utils::git;

/// 验证 GitHub 模板配置
pub fn validate_git_template(template: &GitTemplate) -> Result<(), ScxVoidError> {
    // 验证 URL 格式
    git::parse_github_url(&template.repository_url)?;

    // 验证分支名（如果不为空）
    if !template.default_branch.is_empty() && !is_valid_branch_name(&template.default_branch) {
        return Err(ScxVoidError::GitBranchNotFound(
            template.default_branch.clone(),
        ));
    }

    Ok(())
}

/// 验证分支名格式
pub fn is_valid_branch_name(branch: &str) -> bool {
    if branch.is_empty() {
        return false;
    }

    if branch.starts_with('-') {
        return false;
    }

    if branch.contains("..") {
        return false;
    }

    if branch.contains(".lock") {
        return false;
    }

    let invalid_chars = ['~', '^', ':', '\\', '?', '*', '[', ']', '@'];
    for ch in invalid_chars.iter() {
        if branch.contains(*ch) {
            return false;
        }
    }

    true
}

/// 验证模板 ID 是否存在
#[allow(dead_code)]
pub fn validate_template_id(template_id: &str) -> Result<(), ScxVoidError> {
    if !registry::template_exists(template_id) {
        return Err(ScxVoidError::GitTemplateNotFound(template_id.to_string()));
    }
    Ok(())
}

/// 验证模板路径
#[allow(dead_code)]
pub fn validate_template_path(path: &str) -> Result<(), ScxVoidError> {
    if path.is_empty() {
        return Ok(());
    }

    if !path.starts_with('/') && !path.starts_with("./") {
        return Err(ScxVoidError::InvalidGitHubUrl(format!(
            "无效的模板路径 '{}': 必须以 / 或 ./ 开头",
            path
        )));
    }

    let invalid_chars = ['\\', '\0'];
    for ch in invalid_chars.iter() {
        if path.contains(*ch) {
            return Err(ScxVoidError::InvalidGitHubUrl(format!(
                "无效的模板路径 '{}': 包含非法字符",
                path
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_branch_name() {
        assert!(is_valid_branch_name("main"));
        assert!(is_valid_branch_name("develop"));
        assert!(is_valid_branch_name("feature/new-feature"));
        assert!(is_valid_branch_name("release/v1.0.0"));

        assert!(!is_valid_branch_name(""));
        assert!(!is_valid_branch_name("-invalid"));
        assert!(!is_valid_branch_name("feature..test"));
        assert!(!is_valid_branch_name("branch.lock"));
    }

    #[test]
    fn test_validate_template_path() {
        assert!(validate_template_path("").is_ok());
        assert!(validate_template_path("/").is_ok());
        assert!(validate_template_path("/templates/base").is_ok());
        assert!(validate_template_path("./base").is_ok());

        assert!(validate_template_path("invalid").is_err());
        assert!(validate_template_path("\\windows\\path").is_err());
    }
}
