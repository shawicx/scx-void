pub mod detect;
pub mod git;
pub mod node;
pub mod tauri;
pub mod version;

pub use detect::ProjectKind;
pub use version::BumpKind;

use crate::errors::ScxVoidError;
use std::path::Path;

pub struct ReleaseContext {
    pub new_version: String,
    pub dry_run: bool,
    pub no_push: bool,
    pub no_publish: bool,
}

pub enum ReleaseStep {
    BumpNodeVersion {
        new_version: String,
    },
    BumpTauriVersion {
        new_version: String,
    },
    GitAdd {
        paths: Vec<String>,
    },
    GitCommit {
        message: String,
    },
    GitTag {
        tag: String,
    },
    GitPush {
        branch: String,
    },
    GitPushTag {
        tag: String,
    },
    NpmPublish,
}

impl ReleaseStep {
    pub fn describe(&self) -> String {
        match self {
            Self::BumpNodeVersion { new_version } => {
                format!("更新 package.json 版本为 {}", new_version)
            }
            Self::BumpTauriVersion { new_version } => format!(
                "同步更新 package.json / tauri.conf.json / Cargo.toml 版本为 {}",
                new_version
            ),
            Self::GitAdd { paths } => format!("git add {}", paths.join(" ")),
            Self::GitCommit { message } => format!("git commit -m \"{}\"", message),
            Self::GitTag { tag } => format!("git tag {}", tag),
            Self::GitPush { branch } => format!("git push origin {}", branch),
            Self::GitPushTag { tag } => format!("git push origin {}", tag),
            Self::NpmPublish => "npm publish".to_string(),
        }
    }

    pub fn execute(&self, cwd: &Path) -> Result<(), ScxVoidError> {
        match self {
            Self::BumpNodeVersion { new_version } => version::write_package_version(cwd, new_version),
            Self::BumpTauriVersion { new_version } => {
                version::write_package_version(cwd, new_version)?;
                let src_tauri = cwd.join("src-tauri");
                let conf_updated = version::write_tauri_conf_version(&src_tauri, new_version)?;
                version::write_cargo_toml_version(&src_tauri, new_version)?;
                if !conf_updated {
                    println!("  tauri.conf.json 使用版本引用（../package.json），已跳过");
                }
                println!("  提示: src-tauri/Cargo.lock 未更新，下次构建时自动同步");
                Ok(())
            }
            Self::GitAdd { paths } => {
                let refs: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
                git::add(cwd, &refs)
            }
            Self::GitCommit { message } => {
                git::commit(cwd, message)?;
                // pre-commit hook 可能修改文件导致提交后又出现脏工作区
                if !git::is_clean(cwd)? {
                    return Err(ScxVoidError::GitCommandFailed {
                        command: "git commit".to_string(),
                        reason: "提交后工作区仍有改动（pre-commit hook 修改了文件？），请手动处理后重试"
                            .to_string(),
                    });
                }
                Ok(())
            }
            Self::GitTag { tag } => git::create_tag(cwd, tag),
            Self::GitPush { branch } => git::push_branch(cwd, branch),
            Self::GitPushTag { tag } => git::push_tag(cwd, tag),
            Self::NpmPublish => {
                let output = duct::cmd("npm", &["publish"])
                    .dir(cwd)
                    .run()
                    .map_err(|e| ScxVoidError::GitCommandFailed {
                        command: "npm publish".to_string(),
                        reason: e.to_string(),
                    })?;
                if output.status.success() {
                    Ok(())
                } else {
                    Err(ScxVoidError::GitCommandFailed {
                        command: "npm publish".to_string(),
                        reason: format!("exit status {}", output.status),
                    })
                }
            }
        }
    }
}

/// 版本号确定前的公共检查：git 仓库、已有提交、工作区干净
pub fn common_preflight(cwd: &Path) -> Result<(), ScxVoidError> {
    if !git::is_git_repo(cwd) {
        return Err(ScxVoidError::NotAGitRepository);
    }
    if !git::has_commit(cwd) {
        return Err(ScxVoidError::GitCommandFailed {
            command: "git rev-parse --verify HEAD".to_string(),
            reason: "仓库尚无任何提交，请先完成初始提交".to_string(),
        });
    }
    if !git::is_clean(cwd)? {
        return Err(ScxVoidError::DirtyWorktree(git::status_porcelain(cwd)?));
    }
    Ok(())
}

pub fn execute_steps(steps: &[ReleaseStep], cwd: &Path, dry_run: bool) -> Result<(), ScxVoidError> {
    let total = steps.len();
    let mut completed: Vec<String> = Vec::new();
    for (i, step) in steps.iter().enumerate() {
        println!("[{}/{}] {}", i + 1, total, step.describe());
        if dry_run {
            completed.push(step.describe());
            continue;
        }
        match step.execute(cwd) {
            Ok(()) => completed.push(step.describe()),
            Err(e) => {
                return Err(ScxVoidError::ReleaseStepFailed {
                    step: step.describe(),
                    completed,
                    reason: e.to_string(),
                });
            }
        }
    }
    Ok(())
}
