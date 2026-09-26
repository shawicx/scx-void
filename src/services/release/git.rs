use crate::errors::ScxVoidError;
use std::path::Path;

fn git_failed(command: String, reason: String) -> ScxVoidError {
    ScxVoidError::GitCommandFailed { command, reason }
}

/// 读取型命令：捕获 stdout（duct 在命令失败时把 stderr 带入错误信息）
fn run_git_read(cwd: &Path, args: &[&str]) -> Result<String, ScxVoidError> {
    duct::cmd("git", args)
        .dir(cwd)
        .read()
        .map_err(|e| git_failed(format!("git {}", args.join(" ")), e.to_string()))
}

/// 执行型命令：继承 stdio（credential / hook 交互可见）
fn run_git_run(cwd: &Path, args: &[&str]) -> Result<(), ScxVoidError> {
    let command = format!("git {}", args.join(" "));
    let output = duct::cmd("git", args)
        .dir(cwd)
        .run()
        .map_err(|e| git_failed(command.clone(), e.to_string()))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(git_failed(command, format!("exit status {}", output.status)))
    }
}

pub fn is_git_repo(cwd: &Path) -> bool {
    duct::cmd("git", &["rev-parse", "--git-dir"])
        .dir(cwd)
        .read()
        .is_ok()
}

pub fn has_commit(cwd: &Path) -> bool {
    duct::cmd("git", &["rev-parse", "--verify", "HEAD"])
        .dir(cwd)
        .read()
        .is_ok()
}

pub fn status_porcelain(cwd: &Path) -> Result<String, ScxVoidError> {
    run_git_read(cwd, &["status", "--porcelain"])
}

pub fn is_clean(cwd: &Path) -> Result<bool, ScxVoidError> {
    Ok(status_porcelain(cwd)?.trim().is_empty())
}

/// git branch --show-current 对无提交的新仓库也能返回分支名；
/// detached HEAD 时返回空
pub fn current_branch(cwd: &Path) -> Result<String, ScxVoidError> {
    let branch = run_git_read(cwd, &["branch", "--show-current"])?
        .trim()
        .to_string();
    if branch.is_empty() {
        return Err(git_failed(
            "git branch --show-current".to_string(),
            "HEAD 不在任何分支上（detached HEAD），无法发布".to_string(),
        ));
    }
    Ok(branch)
}

pub fn has_remote(cwd: &Path) -> Result<bool, ScxVoidError> {
    Ok(!run_git_read(cwd, &["remote"])?.trim().is_empty())
}

pub fn tag_exists(cwd: &Path, tag: &str) -> Result<bool, ScxVoidError> {
    let out = run_git_read(cwd, &["tag", "-l", tag])?;
    Ok(out.lines().any(|l| l.trim() == tag))
}

pub fn add(cwd: &Path, paths: &[&str]) -> Result<(), ScxVoidError> {
    let mut args = vec!["add"];
    args.extend_from_slice(paths);
    run_git_run(cwd, &args)
}

pub fn commit(cwd: &Path, message: &str) -> Result<(), ScxVoidError> {
    run_git_run(cwd, &["commit", "-m", message])
}

pub fn create_tag(cwd: &Path, tag: &str) -> Result<(), ScxVoidError> {
    run_git_run(cwd, &["tag", tag])
}

pub fn push_branch(cwd: &Path, branch: &str) -> Result<(), ScxVoidError> {
    run_git_run(cwd, &["push", "origin", branch])
}

pub fn push_tag(cwd: &Path, tag: &str) -> Result<(), ScxVoidError> {
    run_git_run(cwd, &["push", "origin", tag])
}
