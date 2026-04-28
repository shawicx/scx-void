use crate::services::setup::SetupService;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum SetupCommands {
    /// 安装前端开发环境
    Frontend,
}

impl SetupCommands {
    pub async fn run(command: SetupCommands) -> Result<(), Box<dyn std::error::Error>> {
        match command {
            SetupCommands::Frontend => run_frontend_setup().await,
        }
    }
}

async fn run_frontend_setup() -> Result<(), Box<dyn std::error::Error>> {
    let service = SetupService::frontend();
    let items = service.installer_names();

    let selection_items: Vec<String> = items
        .iter()
        .map(|(name, status)| match status {
            Some(ver) => format!("{} [已安装: {}]", name, ver),
            None => name.clone(),
        })
        .collect();

    let selected = dialoguer::MultiSelect::new()
        .with_prompt("选择要安装的前端开发工具")
        .items(&selection_items)
        .interact()?;

    if selected.is_empty() {
        println!("未选择任何组件");
        return Ok(());
    }

    let selected_names: Vec<&str> = selected.iter().map(|&idx| items[idx].0.as_str()).collect();
    println!("\n即将安装: {}", selected_names.join(", "));

    if !dialoguer::Confirm::new()
        .with_prompt("确认安装？")
        .interact()?
    {
        println!("已取消");
        return Ok(());
    }

    let results = service.install_selected(&selected);
    let failed = results.iter().filter(|r| r.is_err()).count();
    if failed > 0 {
        eprintln!("{} 个组件安装失败", failed);
    } else {
        println!("所有组件安装完成！");
    }
    Ok(())
}
