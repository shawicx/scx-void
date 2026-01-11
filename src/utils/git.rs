use crate::errors::ScxVoidError;
use duct::cmd;
use std::process::Output;

/// 检查系统是否安装了 Git
pub fn is_git_installed() -> bool {
    match cmd!("git", "--version").run() {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// 验证 Git URL 格式
pub fn validate_git_url(url: &str) -> bool {
    // 支持的 URL 格式：
    // - https://github.com/user/repo.git
    // - http://github.com/user/repo.git
    // - git@github.com:user/repo.git
    // - github.com/user/repo.git

    if url.is_empty() {
        return false;
    }

    // HTTPS URL
    if url.starts_with("https://") || url.starts_with("http://") {
        return url.contains(".git") || url.contains("github.com") || url.contains("gitlab.com");
    }

    // SSH URL (git@)
    if url.starts_with("git@") {
        return url.contains(":") && url.contains(".git");
    }

    // 简短格式 (user/repo 或 owner/repo)
    if !url.contains('/') {
        return false;
    }

    true
}

/// 获取 Git 仓库的默认分支
pub fn get_default_branch(repository_url: &str) -> Result<String, ScxVoidError> {
    let output = cmd!("git", "remote", "show", repository_url)
        .read()
        .map_err(|e| ScxVoidError::GitCloneError(format!("无法获取仓库信息: {}", e)))?;

    let stdout = output.as_str();

    // 解析输出中的 HEAD 分支
    for line in stdout.lines() {
        if line.contains("HEAD branch:") {
            if let Some(branch) = line.split(':').nth(1) {
                return Ok(branch.trim().to_string());
            }
        }
    }

    // 默认返回 main
    Ok("main".to_string())
}

/// 检查 Git 仓库的分支是否存在
pub fn branch_exists(repository_url: &str, branch: &str) -> Result<bool, ScxVoidError> {
    let output = cmd!("git", "ls-remote", "--heads", repository_url, branch)
        .read()
        .map_err(|e| ScxVoidError::GitCloneError(format!("无法检查分支: {}", e)))?;

    let stdout = output.as_str();
    Ok(!stdout.trim().is_empty())
}

/// 获取 Git 仓库的所有分支
pub fn list_branches(repository_url: &str) -> Result<Vec<String>, ScxVoidError> {
    let output = cmd!("git", "ls-remote", "--heads", repository_url)
        .read()
        .map_err(|e| ScxVoidError::GitCloneError(format!("无法列出分支: {}", e)))?;

    let stdout = output.as_str();
    let branches: Vec<String> = stdout
        .lines()
        .filter_map(|line| {
            line.split('\t')
                .nth(1)
                .and_then(|s| s.strip_prefix("refs/heads/"))
        })
        .map(String::from)
        .collect();

    Ok(branches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_git_url_https() {
        assert!(validate_git_url("https://github.com/user/repo.git"));
        assert!(validate_git_url("http://github.com/user/repo.git"));
    }

    #[test]
    fn test_validate_git_url_ssh() {
        assert!(validate_git_url("git@github.com:user/repo.git"));
    }

    #[test]
    fn test_validate_git_url_invalid() {
        assert!(!validate_git_url(""));
        assert!(!validate_git_url("not-a-url"));
        assert!(!validate_git_url("no slashes"));
    }

    #[test]
    fn test_validate_git_url_short_format() {
        assert!(validate_git_url("user/repo"));
        assert!(validate_git_url("owner/repo.git"));
    }
}
