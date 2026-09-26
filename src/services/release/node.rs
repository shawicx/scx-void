use super::git;
use super::version;
use super::{ReleaseContext, ReleaseStep};
use crate::errors::ScxVoidError;
use std::path::Path;

fn check_npm() -> Result<(), ScxVoidError> {
    duct::cmd("npm", &["--version"])
        .read()
        .map(|_| ())
        .map_err(|_| ScxVoidError::ReleaseToolNotFound {
            tool: "npm".to_string(),
            hint: "请先安装 Node.js（含 npm）".to_string(),
        })
}

pub fn plan(cwd: &Path, ctx: &ReleaseContext) -> Result<Vec<ReleaseStep>, ScxVoidError> {
    let tag = format!("v{}", ctx.new_version);
    if git::tag_exists(cwd, &tag)? {
        return Err(ScxVoidError::TagAlreadyExists(tag));
    }
    if !ctx.dry_run && !ctx.no_push && !git::has_remote(cwd)? {
        return Err(ScxVoidError::GitRemoteMissing);
    }
    if version::package_is_private(cwd) {
        return Err(ScxVoidError::PrivatePackage);
    }
    if !ctx.dry_run && !ctx.no_publish {
        check_npm()?;
    }

    let branch = git::current_branch(cwd)?;
    let mut steps = vec![
        ReleaseStep::BumpNodeVersion {
            new_version: ctx.new_version.clone(),
        },
        ReleaseStep::GitAdd {
            paths: vec!["package.json".to_string()],
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
    if !ctx.no_publish {
        steps.push(ReleaseStep::NpmPublish);
    }
    Ok(steps)
}
