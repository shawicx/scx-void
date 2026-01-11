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
pub async fn clone_repository(options: CloneOptions) -> Result<(), ScxVoidError> {
    let mut args = vec!["clone".to_string()];

    if let Some(branch) = &options.branch {
        args.push("--branch".to_string());
        args.push(branch.clone());
    }

    if let Some(depth) = options.depth {
        args.push("--depth".to_string());
        args.push(depth.to_string());
    }

    args.push(options.repository_url.clone());
    args.push(options.target_dir.clone());

    println!("正在克隆仓库: {}", options.repository_url);

    let git_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let output = cmd("git", &git_args)
        .run()
        .map_err(|e| ScxVoidError::GitCloneError(format!("克隆失败: {}", e)))?;

    if !output.status.success() {
        return Err(ScxVoidError::GitCloneError("Git 命令执行失败".to_string()));
    }

    Ok(())
}

/// 克隆仓库的特定子目录（使用稀疏检出）
pub async fn clone_sparse_checkout(
    options: CloneOptions,
    sparse_path: &str,
) -> Result<(), ScxVoidError> {
    let target_dir = &options.target_dir;

    println!("正在使用稀疏检出克隆: {}", sparse_path);

    cmd(
        "git",
        [
            "clone",
            "--depth",
            "1",
            "--filter=blob:none",
            "--sparse",
            &options.repository_url,
            target_dir,
        ],
    )
    .run()
    .map_err(|e| ScxVoidError::GitCloneError(format!("稀疏检出克隆失败: {}", e)))?;

    cmd("git", ["sparse-checkout", "set", sparse_path])
        .dir(target_dir)
        .run()
        .map_err(|e| ScxVoidError::GitCloneError(format!("设置稀疏检出路径失败: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone_repository_build() {
        let options = CloneOptions::new(
            "https://github.com/test/repo.git".to_string(),
            "/tmp/test-repo".to_string(),
        );

        assert_eq!(options.repository_url, "https://github.com/test/repo.git");
        assert_eq!(options.target_dir, "/tmp/test-repo");
    }

    #[test]
    fn test_clone_options_with_branch() {
        let options = CloneOptions::new(
            "https://github.com/test/repo.git".to_string(),
            "/tmp/test-repo".to_string(),
        )
        .with_branch("develop".to_string());

        assert_eq!(options.branch, Some("develop".to_string()));
    }

    #[test]
    fn test_clone_options_with_depth() {
        let options = CloneOptions::new(
            "https://github.com/test/repo.git".to_string(),
            "/tmp/test-repo".to_string(),
        )
        .with_depth(1);

        assert_eq!(options.depth, Some(1));
    }
}
