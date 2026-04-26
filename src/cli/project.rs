use clap::Subcommand;
use dialoguer::{Input, Select};

#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    /// 初始化一个新项目
    Init {
        /// GitHub 模板 ID（预定义模板的唯一标识）
        #[arg(short, long)]
        template: Option<String>,

        /// 自定义 GitHub 仓库（owner/repo 格式）
        #[arg(short, long)]
        url: Option<String>,

        /// Git 分支或标签
        #[arg(short, long)]
        branch: Option<String>,

        /// 项目名称
        #[arg(short, long)]
        name: Option<String>,
    },
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
            ProjectCommands::Init {
                template,
                url,
                branch,
                name,
            } => {
                init_project(template, url, branch, name).await;
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

async fn init_project(
    template: Option<String>,
    url: Option<String>,
    branch: Option<String>,
    name: Option<String>,
) {
    println!("正在初始化一个新项目...");

    let project_name =
        name.unwrap_or_else(|| Input::new().with_prompt("输入项目名称").interact().unwrap());

    let use_command_mode = template.is_some() || url.is_some() || branch.is_some();

    if use_command_mode {
        handle_command_mode(project_name, template, url, branch).await;
    } else {
        handle_interactive_mode(project_name).await;
    }
}

async fn handle_command_mode(
    project_name: String,
    template: Option<String>,
    url: Option<String>,
    branch: Option<String>,
) {
    use crate::services::project::git::registry;
    use crate::services::project::git::types::GitTemplate;

    let git_template = if let Some(template_id) = template {
        registry::get_template_by_id(&template_id).unwrap_or_else(|| {
            eprintln!("模板 '{}' 不存在", template_id);
            std::process::exit(1);
        })
    } else if let Some(repo_url) = url {
        GitTemplate::custom(&repo_url, branch.as_deref(), None)
    } else {
        eprintln!("请指定 --template 或 --url 参数");
        std::process::exit(1);
    };

    match crate::services::project::create_project(&project_name, &git_template, branch.as_deref())
        .await
    {
        Ok(_) => println!("项目'{}'创建成功!", project_name),
        Err(e) => {
            eprintln!("创建项目时出错: {}", e);
            std::process::exit(1);
        }
    }
}

async fn handle_interactive_mode(project_name: String) {
    use crate::services::project::git::registry;
    use crate::services::project::git::types::GitTemplate;

    let templates = registry::get_all_templates();
    let mut display_items: Vec<String> = templates
        .iter()
        .map(|t| format!("{} - {}", t.display_name, t.description))
        .collect();

    display_items.push("自定义 GitHub 仓库".to_string());

    let selection = Select::new()
        .with_prompt("选择项目模板")
        .items(&display_items)
        .default(0)
        .interact()
        .unwrap();

    let (git_template, branch) = if selection < templates.len() {
        (templates[selection].clone(), None as Option<String>)
    } else {
        let url: String = Input::new()
            .with_prompt("输入 GitHub 仓库（owner/repo 格式）")
            .interact()
            .unwrap();

        let branch_input: String = Input::new()
            .with_prompt("输入分支名（默认: main）")
            .allow_empty(true)
            .interact()
            .unwrap();

        let branch = if branch_input.is_empty() {
            None
        } else {
            Some(branch_input)
        };

        (GitTemplate::custom(&url, branch.as_deref(), None), branch)
    };

    match crate::services::project::create_project(&project_name, &git_template, branch.as_deref())
        .await
    {
        Ok(_) => println!("项目'{}'创建成功!", project_name),
        Err(e) => {
            eprintln!("创建项目时出错: {}", e);
            std::process::exit(1);
        }
    }
}

async fn add_to_project(stack_type: String) {
    println!("正在向项目添加{}...", stack_type);
    println!("添加{}功能尚未实现。", stack_type);
}

async fn manage_ai_rule(template: String, force: bool, interactive: bool) {
    println!("正在管理 Ai Code 规则文件...");

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

    match crate::services::project::ai_rule::manage_ai_rule_file(&final_template, force).await {
        Ok(_) => println!("Ai Code 规则文件处理成功!"),
        Err(e) => {
            eprintln!("处理 Ai Code 规则文件时出错: {}", e);
            std::process::exit(1);
        }
    }
}
