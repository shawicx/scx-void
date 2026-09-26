use crate::errors::ScxVoidError;
use crate::services::release::{self, ProjectKind, ReleaseContext};
use clap::Args;
use dialoguer::Confirm;
use semver::Version;

#[derive(Args, Debug)]
pub struct ReleaseArgs {
    /// 目标项目类型（node/tauri）。未提供时自动检测
    #[arg(short, long, value_name = "TYPE")]
    pub r#type: Option<String>,

    /// 递增 patch 版本（1.2.3 → 1.2.4）
    #[arg(long, group = "bump")]
    pub patch: bool,

    /// 递增 minor 版本（1.2.3 → 1.3.0）
    #[arg(long, group = "bump")]
    pub minor: bool,

    /// 递增 major 版本（1.2.3 → 2.0.0）
    #[arg(long, group = "bump")]
    pub major: bool,

    /// 演练模式：打印将执行的步骤，不实际执行
    #[arg(long = "dry-run")]
    pub dry_run: bool,

    /// 跳过发布确认
    #[arg(short, long)]
    pub yes: bool,

    /// 跳过 git push
    #[arg(long)]
    pub no_push: bool,

    /// 跳过 npm publish
    #[arg(long)]
    pub no_publish: bool,
}

pub async fn run_release(args: ReleaseArgs) -> Result<(), ScxVoidError> {
    let cwd = std::env::current_dir()
        .map_err(|e| ScxVoidError::GeneralError(format!("无法获取当前目录: {}", e)))?;

    let kind = release::detect::detect(&cwd, args.r#type.as_deref())?;
    println!("检测到项目类型: {}", kind.as_str());

    release::common_preflight(&cwd)?;

    let current = release::version::read_package_version(&cwd)?;
    println!("当前版本: {}", current);

    let bump_kind = if args.patch {
        release::BumpKind::Patch
    } else if args.minor {
        release::BumpKind::Minor
    } else if args.major {
        release::BumpKind::Major
    } else {
        prompt_bump(&current)?
    };
    let new = release::version::bump(&current, bump_kind);
    println!("目标版本: {} → {}", current, new);

    let ctx = ReleaseContext {
        new_version: new.to_string(),
        dry_run: args.dry_run,
        no_push: args.no_push,
        no_publish: args.no_publish,
    };
    let steps = match kind {
        ProjectKind::Node => release::node::plan(&cwd, &ctx)?,
        ProjectKind::Tauri => release::tauri::plan(&cwd, &ctx)?,
    };

    if args.dry_run {
        println!("\n[dry-run] 以下步骤不会实际执行:");
    } else if !args.yes {
        let confirmed = Confirm::new()
            .with_prompt(format!("确认发布 v{}（{}）", new, kind.as_str()))
            .interact()
            .map_err(|e| ScxVoidError::GeneralError(e.to_string()))?;
        if !confirmed {
            println!("已取消");
            return Ok(());
        }
    }

    release::execute_steps(&steps, &cwd, args.dry_run)?;

    if args.dry_run {
        println!("\n[dry-run] 演练完成，未修改任何文件");
    } else {
        println!("\n发布完成: v{}", new);
    }
    Ok(())
}

fn prompt_bump(current: &Version) -> Result<release::BumpKind, ScxVoidError> {
    use dialoguer::Select;

    let candidates = [
        release::BumpKind::Patch,
        release::BumpKind::Minor,
        release::BumpKind::Major,
    ];
    let items: Vec<String> = candidates
        .iter()
        .map(|k| format!("{}: {} → {}", k.as_str(), current, release::version::bump(current, *k)))
        .collect();
    let selected = Select::new()
        .with_prompt("选择版本递增类型")
        .items(&items)
        .default(0)
        .interact()
        .map_err(|e| ScxVoidError::GeneralError(e.to_string()))?;
    Ok(candidates[selected])
}
