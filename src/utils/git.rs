use crate::errors::ScxVoidError;

/// 解析 GitHub URL 或短格式为 (owner, repo)
/// 支持: "https://github.com/owner/repo", "owner/repo", "owner/repo.git"
pub fn parse_github_url(url: &str) -> Result<(String, String), ScxVoidError> {
    let trimmed = url.trim().trim_end_matches('/');

    // 去掉 https://github.com/ 前缀
    let path = if let Some(rest) = trimmed.strip_prefix("https://github.com/") {
        rest
    } else if let Some(rest) = trimmed.strip_prefix("http://github.com/") {
        rest
    } else if let Some(rest) = trimmed.strip_prefix("github.com/") {
        rest
    } else {
        trimmed
    };

    // 去掉 .git 后缀
    let path = path.strip_suffix(".git").unwrap_or(path);

    let parts: Vec<&str> = path.splitn(3, '/').collect();
    if parts.len() < 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err(ScxVoidError::InvalidGitHubUrl(format!(
            "无效的 GitHub URL: {}",
            url
        )));
    }

    Ok((parts[0].to_string(), parts[1].to_string()))
}

/// 构建 GitHub 归档下载 URL
pub fn build_archive_url(owner: &str, repo: &str, branch: &str) -> String {
    format!(
        "https://codeload.github.com/{}/{}/zip/refs/heads/{}",
        owner, repo, branch
    )
}

/// 验证 GitHub URL 格式
#[allow(dead_code)]
pub fn validate_git_url(url: &str) -> bool {
    parse_github_url(url).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_github_url_https() {
        assert_eq!(
            parse_github_url("https://github.com/shawicx/template-node-ts-cli").unwrap(),
            ("shawicx".to_string(), "template-node-ts-cli".to_string())
        );
    }

    #[test]
    fn test_parse_github_url_short() {
        assert_eq!(
            parse_github_url("shawicx/template-vue3-standard").unwrap(),
            ("shawicx".to_string(), "template-vue3-standard".to_string())
        );
    }

    #[test]
    fn test_parse_github_url_with_git_suffix() {
        assert_eq!(
            parse_github_url("https://github.com/shawicx/template-react-modern.git").unwrap(),
            ("shawicx".to_string(), "template-react-modern".to_string())
        );
    }

    #[test]
    fn test_parse_github_url_invalid() {
        assert!(parse_github_url("").is_err());
        assert!(parse_github_url("no-slash").is_err());
        assert!(parse_github_url("/only-repo").is_err());
    }

    #[test]
    fn test_build_archive_url() {
        let url = build_archive_url("shawicx", "template-node-ts-cli", "main");
        assert_eq!(
            url,
            "https://codeload.github.com/shawicx/template-node-ts-cli/zip/refs/heads/main"
        );
    }

    #[test]
    fn test_validate_git_url() {
        assert!(validate_git_url("shawicx/template-node-ts-cli"));
        assert!(validate_git_url("https://github.com/shawicx/template-node-ts-cli"));
        assert!(!validate_git_url(""));
        assert!(!validate_git_url("invalid"));
    }
}
