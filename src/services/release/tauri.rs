use super::git;
use super::{ReleaseContext, ReleaseStep};
use crate::errors::ScxVoidError;
use std::path::Path;

pub fn plan(cwd: &Path, ctx: &ReleaseContext) -> Result<Vec<ReleaseStep>, ScxVoidError> {
    let src_tauri = cwd.join("src-tauri");
    for file in ["tauri.conf.json", "Cargo.toml"] {
        let path = src_tauri.join(file);
        if !path.exists() {
            return Err(ScxVoidError::ReleaseProjectNotDetected(format!(
                "Tauri 项目缺少 {}",
                path.display()
            )));
        }
    }

    let tag = format!("v{}", ctx.new_version);
    if git::tag_exists(cwd, &tag)? {
        return Err(ScxVoidError::TagAlreadyExists(tag));
    }
    if !ctx.dry_run && !ctx.no_push && !git::has_remote(cwd)? {
        return Err(ScxVoidError::GitRemoteMissing);
    }

    let branch = git::current_branch(cwd)?;
    let mut steps = vec![
        ReleaseStep::BumpTauriVersion {
            new_version: ctx.new_version.clone(),
        },
        ReleaseStep::GitAdd {
            paths: vec![
                "package.json".to_string(),
                "src-tauri/tauri.conf.json".to_string(),
                "src-tauri/Cargo.toml".to_string(),
            ],
        },
        ReleaseStep::GitCommit {
            message: format!("chore: release {}", tag),
        },
        ReleaseStep::GitTag { tag: tag.clone() },
    ];
    if !ctx.no_push {
        steps.push(ReleaseStep::GitPush { branch });
        steps.push(ReleaseStep::GitPushTag { tag });
    }
    Ok(steps)
}
