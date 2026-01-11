use clap::Subcommand;
use dialoguer::{Input, Select};

#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    /// 初始化一个新项目
    Init,
    /// 向项目添加特定技术栈
    Add {
        #[arg(help = "要添加的技术栈类型")]
        stack_type: String,
    },
    /// 生成或更新 AGENTS.md 文件
    AiRule {
        /// 模板类型 (basic, advanced)
        #[arg(short, long, default_value = "advanced")]
        template: String,

        /// 强制覆盖现有文件
        #[arg(short, long)]
        force: bool,

        /// 交互式配置
        #[arg(short, long)]
        interactive: bool,
    },
}

impl ProjectCommands {
    pub async fn run(command: ProjectCommands) {
        match command {
            ProjectCommands::Init => {
                init_project().await;
            }
            ProjectCommands::Add { stack_type } => {
                add_to_project(stack_type).await;
            }
            ProjectCommands::AiRule {
                template,
                force,
                interactive,
            } => {
                manage_ai_rule(template, force, interactive).await;
            }
        }
    }
}

async fn init_project() {
    println!("正在初始化一个新项目...");

    // 从用户获取项目名称
    let project_name: String = Input::new().with_prompt("输入项目名称").interact().unwrap();

    // 从用户获取项目类型
    let project_types = ["Node TypeScript", "React", "Vue", "NestJS", "NextJS"];

    let selection = Select::new()
        .with_prompt("选择项目类型")
        .items(&project_types)
        .default(0)
        .interact()
        .unwrap();

    println!("正在创建{}项目: {}", project_types[selection], project_name);

    // 委托项目服务创建项目
    match crate::services::project::create_project(&project_name, selection).await {
        Ok(_) => println!("项目'{}'创建成功!", project_name),
        Err(e) => eprintln!("创建项目时出错: {}", e),
    }
}

async fn add_to_project(stack_type: String) {
    println!("正在向项目添加{}...", stack_type);
    // 添加技术栈到现有项目的实现
    println!("添加{}功能尚未实现。", stack_type);
}

async fn manage_ai_rule(template: String, force: bool, interactive: bool) {
    println!("正在管理 Ai Code 规则文件...");

    // 交互式配置选项
    let final_template = if interactive {
        let templates = ["advanced", "basic"];
        let selection = Select::new()
            .with_prompt("选择模板类型")
            .items(&templates)
            .default(0)
            .interact()
            .unwrap();
        templates[selection].to_string()
    } else {
        template
    };

    // 委托服务层处理Ai规则文件
    match crate::services::project::ai_rule::manage_ai_rule_file(&final_template, force)
        .await
    {
        Ok(_) => println!("Ai Code 规则文件处理成功!"),
        Err(e) => {
            eprintln!("处理 Ai Code 规则文件时出错: {}", e);
            std::process::exit(1);
        }
    }
}
