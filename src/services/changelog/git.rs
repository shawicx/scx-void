use crate::errors::ScxVoidError;
use semver::Version;
use std::path::Path;

fn git_failed(command: String, reason: String) -> ScxVoidError {
    ScxVoidError::GitCommandFailed { command, reason }
}

fn run_git_read(cwd: &Path, args: &[&str]) -> Result<String, ScxVoidError> {
    duct::cmd("git", args)
        .dir(cwd)
        .read()
        .map_err(|e| git_failed(format!("git {}", args.join(" ")), e.to_string()))
}

/// 一个用于生成 CHANGELOG 的提交条目
#[derive(Debug, Clone)]
pub struct Commit {
    /// 短哈希（用于渲染与去重）
    pub short: String,
    /// 提交主题（第一行）
    pub subject: String,
}

/// 是否位于 git 仓库内
pub fn is_git_repo(cwd: &Path) -> bool {
    duct::cmd("git", &["rev-parse", "--git-dir"])
        .dir(cwd)
        .read()
        .is_ok()
}

/// 提取符合 semver 的版本 tag，按版本号升序返回（tag 名保留原始写法）
pub fn version_tags(cwd: &Path) -> Result<Vec<(String, Version)>, ScxVoidError> {
    let out = run_git_read(cwd, &["tag", "-l"])?;
    let mut tags: Vec<(String, Version)> = out
        .lines()
        .filter_map(|line| {
            let name = line.trim();
            if name.is_empty() {
                return None;
            }
            Version::parse(name.trim_start_matches('v'))
                .ok()
                .map(|v| (name.to_string(), v))
        })
        .collect();
    tags.sort_by(|a, b| a.1.cmp(&b.1));
    Ok(tags)
}

/// 读取 tag 所在提交的日期（YYYY-MM-DD）
pub fn tag_date(cwd: &Path, tag: &str) -> Result<String, ScxVoidError> {
    run_git_read(cwd, &["log", "-1", "--format=%cs", tag])
}

/// 提取区间内的提交（不含 merge commit），返回按时间新→旧排列。
/// range 为 None 时取全部历史；range 为 Some 时形如 "v1.0.0..v1.1.0" 或 "v1.1.0..HEAD"
pub fn commits_in_range(cwd: &Path, range: Option<&str>) -> Result<Vec<Commit>, ScxVoidError> {
    let mut args: Vec<&str> = vec![
        "log",
        "--no-merges",
        "--pretty=%H\x1f%h\x1f%s",
    ];
    if let Some(r) = range {
        args.push(r);
    }
    let out = run_git_read(cwd, &args)?;
    Ok(out
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| {
            let mut parts = l.splitn(3, '\x1f');
            let hash = parts.next()?.trim().to_string();
            let short = parts.next()?.trim().to_string();
            let subject = parts.next()?.trim().to_string();
            if hash.is_empty() || subject.is_empty() {
                return None;
            }
            Some(Commit { short, subject })
        })
        .collect())
}
