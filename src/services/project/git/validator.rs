use crate::errors::ScxVoidError;
use crate::services::project::git::registry;
use crate::services::project::git::types::GitTemplate;
use crate::utils::git;

/// 验证 Git 模板配置
pub fn validate_git_template(template: &GitTemplate) -> Result<(), ScxVoidError> {
    // 验证 URL
    if !git::validate_git_url(&template.repository_url) {
        return Err(ScxVoidError::InvalidGitUrl(template.repository_url.clone()));
    }

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

    // Git 分支名规则：
    // - 不能以 - 开头
    // - 不能包含 .lock
    // - 不能包含连续的 ..
    // - 不能包含特殊字符：~ ^ : \ ? * [ 等

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
pub fn validate_template_id(template_id: &str) -> Result<(), ScxVoidError> {
    if !registry::template_exists(template_id) {
        return Err(ScxVoidError::GitTemplateNotFound(template_id.to_string()));
    }
    Ok(())
}

/// 验证模板路径
pub fn validate_template_path(path: &str) -> Result<(), ScxVoidError> {
    if path.is_empty() {
        return Ok(()); // 空路径表示根目录，是有效的
    }

    // 检查路径格式
    if !path.starts_with('/') && !path.starts_with("./") {
        return Err(ScxVoidError::InvalidGitUrl(format!(
            "无效的模板路径 '{}': 必须以 / 或 ./ 开头",
            path
        )));
    }

    // 检查是否包含非法字符
    let invalid_chars = ['\\', '\0'];
    for ch in invalid_chars.iter() {
        if path.contains(*ch) {
            return Err(ScxVoidError::InvalidGitUrl(format!(
                "无效的模板路径 '{}': 包含非法字符",
                path
            )));
        }
    }

    Ok(())
}

/// 验证克隆选项
pub fn validate_clone_options(
    repository_url: &str,
    branch: Option<&str>,
    depth: Option<u32>,
) -> Result<(), ScxVoidError> {
    // 验证 URL
    if !git::validate_git_url(repository_url) {
        return Err(ScxVoidError::InvalidGitUrl(repository_url.to_string()));
    }

    // 验证分支
    if let Some(branch_name) = branch {
        if !is_valid_branch_name(branch_name) {
            return Err(ScxVoidError::GitBranchNotFound(branch_name.to_string()));
        }
    }

    // 验证深度
    if let Some(d) = depth {
        if d == 0 {
            return Err(ScxVoidError::GeneralError("克隆深度必须大于 0".to_string()));
        }
        if d > 1000 {
            return Err(ScxVoidError::GeneralError(
                "克隆深度不能超过 1000".to_string(),
            ));
        }
    }

    Ok(())
}

/// 检查 Git 是否安装
pub fn check_git_installed() -> Result<(), ScxVoidError> {
    if !git::is_git_installed() {
        return Err(ScxVoidError::GitNotInstalled);
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

    #[test]
    fn test_validate_clone_options() {
        assert!(validate_clone_options("https://github.com/user/repo.git", None, None).is_ok());
        assert!(
            validate_clone_options("https://github.com/user/repo.git", Some("main"), Some(1))
                .is_ok()
        );

        assert!(validate_clone_options("invalid-url", None, None).is_err());
        assert!(
            validate_clone_options("https://github.com/user/repo.git", Some("-invalid"), None)
                .is_err()
        );
        assert!(validate_clone_options("https://github.com/user/repo.git", None, Some(0)).is_err());
    }
}
