use crate::errors::ScxVoidError;
use crate::services::changelog::{self, ChangelogContext};
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ChangelogSubCommands {
    /// 首次生成：按版本 tag 切割全部提交，生成完整 CHANGELOG（新版本在前）
    Init {
        /// 输出文件路径（默认 CHANGELOG.md）
        #[arg(short = 'o', long, value_name = "FILE")]
        file: Option<String>,

        /// 已存在输出文件时强制重新生成
        #[arg(short, long)]
        force: bool,

        /// 演练模式：打印将生成的内容，不实际写入
        #[arg(long = "dry-run")]
        dry_run: bool,
    },
    /// 增量更新：合并新增版本 tag 与未发布提交到现有 CHANGELOG（幂等，可重复执行）
    Update {
        /// 输出文件路径（默认 CHANGELOG.md）
        #[arg(short, long, value_name = "FILE")]
        file: Option<String>,

        /// 演练模式：打印将生成的内容，不实际写入
        #[arg(long = "dry-run")]
        dry_run: bool,
    },
}

pub async fn run_changelog(command: ChangelogSubCommands) -> Result<(), ScxVoidError> {
    let cwd = std::env::current_dir()
        .map_err(|e| ScxVoidError::GeneralError(format!("无法获取当前目录: {}", e)))?;

    if !changelog::git::is_git_repo(&cwd) {
        return Err(ScxVoidError::NotAGitRepository);
    }

    match command {
        ChangelogSubCommands::Init {
            file,
            force,
            dry_run,
        } => {
            let ctx = ChangelogContext {
                file: file.unwrap_or_else(|| changelog::CHANGELOG_FILE.to_string()),
                dry_run,
            };
            changelog::init(&cwd, &ctx, force)?;
        }
        ChangelogSubCommands::Update { file, dry_run } => {
            let ctx = ChangelogContext {
                file: file.unwrap_or_else(|| changelog::CHANGELOG_FILE.to_string()),
                dry_run,
            };
            changelog::update(&cwd, &ctx)?;
        }
    }
    Ok(())
}
