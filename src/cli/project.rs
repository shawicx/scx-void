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
        /// 技术栈类型 (vue3/react/nextjs/node-cli/nestjs/tauri/java)
        #[arg(short, long)]
        r#type: Option<String>,

        /// 强制覆盖现有文件
        #[arg(short, long)]
        force: bool,
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
            ProjectCommands::AiRule { r#type, force } => {
                manage_ai_rule(r#type, force).await;
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
        Ok(_) => {
            println!("项目'{}'创建成功!", project_name);

            // 按模板的 project_type 生成 AGENTS.md 到新项目目录（不阻断主流程）
            let agents_path = std::path::Path::new(&project_name).join("AGENTS.md");
            let service = crate::services::project::ai_rule::AiRuleService::new();
            if let Err(e) = service
                .manage_rule_file(git_template.project_type, &agents_path)
                .await
            {
                eprintln!("警告：AGENTS.md 生成失败: {}", e);
            }
        }
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
        Ok(_) => {
            println!("项目'{}'创建成功!", project_name);

            // 按模板的 project_type 生成 AGENTS.md 到新项目目录（不阻断主流程）
            let agents_path = std::path::Path::new(&project_name).join("AGENTS.md");
            let service = crate::services::project::ai_rule::AiRuleService::new();
            if let Err(e) = service
                .manage_rule_file(git_template.project_type, &agents_path)
                .await
            {
                eprintln!("警告：AGENTS.md 生成失败: {}", e);
            }
        }
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

async fn manage_ai_rule(type_id: Option<String>, force: bool) {
    use crate::services::project::git::types::ProjectType;

    println!("正在管理 Ai Code 规则文件...");

    let project_type = match type_id {
        Some(id) => match ProjectType::from_ai_rule_id(&id) {
            Some(pt) => pt,
            None => {
                eprintln!(
                    "错误：未知的技术栈类型 '{}'\n可用类型：vue3, react, nextjs, node-cli, nestjs, tauri, java",
                    id
                );
                std::process::exit(1);
            }
        },
        None => prompt_stack_selection(),
    };

    match crate::services::project::ai_rule::manage_ai_rule_file(project_type, force).await {
        Ok(_) => println!("Ai Code 规则文件处理成功!"),
        Err(e) => {
            eprintln!("处理 Ai Code 规则文件时出错: {}", e);
            std::process::exit(1);
        }
    }
}

fn prompt_stack_selection() -> crate::services::project::git::types::ProjectType {
    use crate::services::project::git::types::ProjectType;

    let stacks = [
        ("vue3", "Vue 3 + TypeScript + Vite"),
        ("react", "React 18 + TypeScript + Vite"),
        ("nextjs", "NextJS 14 + App Router + TypeScript"),
        ("node-cli", "Node.js + TypeScript CLI"),
        ("nestjs", "NestJS RESTful API"),
        ("tauri", "Tauri 桌面应用 (Rust + 前端)"),
        ("java", "Java (Maven/Gradle/Spring)"),
    ];
    let items: Vec<String> = stacks
        .iter()
        .map(|(id, desc)| format!("{} - {}", id, desc))
        .collect();

    let selection = Select::new()
        .with_prompt("选择技术栈")
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    ProjectType::from_ai_rule_id(stacks[selection].0).unwrap()
}
