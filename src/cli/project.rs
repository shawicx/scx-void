use clap::Subcommand;
use dialoguer::{Input, Select};

#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    /// 初始化一个新项目
    Init {
        /// 模板源 (local 或 git)
        #[arg(short, long)]
        source: Option<String>,

        /// Git 模板 ID（预定义模板的唯一标识）
        #[arg(short, long)]
        template: Option<String>,

        /// 自定义 Git 仓库 URL
        #[arg(short, long)]
        url: Option<String>,

        /// Git 分支或标签
        #[arg(short, long)]
        branch: Option<String>,

        /// 模板在仓库中的相对路径
        #[arg(long)]
        template_path: Option<String>,

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
                source,
                template,
                url,
                branch,
                template_path,
                name,
            } => {
                init_project(source, template, url, branch, template_path, name).await;
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
    source: Option<String>,
    template: Option<String>,
    url: Option<String>,
    branch: Option<String>,
    template_path: Option<String>,
    name: Option<String>,
) {
    println!("正在初始化一个新项目...");

    let project_name =
        name.unwrap_or_else(|| Input::new().with_prompt("输入项目名称").interact().unwrap());

    let use_command_mode = source.is_some()
        || template.is_some()
        || url.is_some()
        || branch.is_some()
        || template_path.is_some();

    if use_command_mode {
        handle_command_mode(project_name, source, template, url, branch, template_path).await;
    } else {
        handle_interactive_mode(project_name).await;
    }
}

async fn handle_command_mode(
    project_name: String,
    source: Option<String>,
    template: Option<String>,
    url: Option<String>,
    branch: Option<String>,
    template_path: Option<String>,
) {
    let source_type = source.unwrap_or_else(|| {
        if url.is_some() || template.is_some() {
            "git".to_string()
        } else {
            "local".to_string()
        }
    });

    match source_type.as_str() {
        "git" => {
            handle_git_template_command_mode(project_name, template, url, branch, template_path)
                .await
        }
        "local" => handle_local_template_command_mode(project_name).await,
        _ => {
            eprintln!("无效的模板源: {}", source_type);
            std::process::exit(1);
        }
    }
}

async fn handle_git_template_command_mode(
    project_name: String,
    template: Option<String>,
    url: Option<String>,
    branch: Option<String>,
    template_path: Option<String>,
) {
    use crate::services::project::git::registry;
    use crate::services::project::git::types::{GitTemplate, TemplateSource};

    let git_template = if let Some(template_id) = template {
        let template_obj = registry::get_template_by_id(&template_id).unwrap_or_else(|| {
            eprintln!("模板 '{}' 不存在", template_id);
            std::process::exit(1);
        });
        template_obj
    } else if let Some(repo_url) = url {
        GitTemplate::custom(&repo_url, branch.as_deref(), template_path.as_deref())
    } else {
        eprintln!("请指定 --template 或 --url 参数");
        std::process::exit(1);
    };

    let source = TemplateSource::Git(git_template);

    match crate::services::project::create_project_with_source(&project_name, source).await {
        Ok(_) => println!("项目'{}'创建成功!", project_name),
        Err(e) => {
            eprintln!("创建项目时出错: {}", e);
            std::process::exit(1);
        }
    }
}

async fn handle_local_template_command_mode(project_name: String) {
    let project_types = ["Node TypeScript", "React", "Vue", "NestJS", "NextJS"];

    let selection = Select::new()
        .with_prompt("选择项目类型")
        .items(&project_types)
        .default(0)
        .interact()
        .unwrap();

    println!("正在创建{}项目: {}", project_types[selection], project_name);

    match crate::services::project::create_project(&project_name, selection).await {
        Ok(_) => println!("项目'{}'创建成功!", project_name),
        Err(e) => {
            eprintln!("创建项目时出错: {}", e);
            std::process::exit(1);
        }
    }
}

async fn handle_interactive_mode(project_name: String) {
    let source_types = ["本地模板", "Git 模板"];

    let source_selection = Select::new()
        .with_prompt("选择模板源")
        .items(&source_types)
        .default(0)
        .interact()
        .unwrap();

    match source_selection {
        0 => handle_local_template_interactive_mode(project_name).await,
        1 => handle_git_template_interactive_mode(project_name).await,
        _ => unreachable!(),
    }
}

async fn handle_local_template_interactive_mode(project_name: String) {
    let project_types = ["Node TypeScript", "React", "Vue", "NestJS", "NextJS"];

    let selection = Select::new()
        .with_prompt("选择项目类型")
        .items(&project_types)
        .default(0)
        .interact()
        .unwrap();

    println!("正在创建{}项目: {}", project_types[selection], project_name);

    match crate::services::project::create_project(&project_name, selection).await {
        Ok(_) => println!("项目'{}'创建成功!", project_name),
        Err(e) => {
            eprintln!("创建项目时出错: {}", e);
            std::process::exit(1);
        }
    }
}

async fn handle_git_template_interactive_mode(project_name: String) {
    use crate::services::project::git::registry;
    use crate::services::project::git::types::{GitTemplate, TemplateSource};

    let templates = registry::get_all_templates();
    let mut template_names: Vec<String> =
        templates.iter().map(|t| t.display_name.clone()).collect();

    template_names.push("自定义仓库".to_string());

    let selection = Select::new()
        .with_prompt("选择 Git 模板")
        .items(&template_names)
        .default(0)
        .interact()
        .unwrap();

    let git_template = if selection < templates.len() {
        let template = &templates[selection];
        template.clone()
    } else {
        let url: String = Input::new()
            .with_prompt("输入 Git 仓库 URL")
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

        let path_input: String = Input::new()
            .with_prompt("输入模板路径（留空表示根目录）")
            .allow_empty(true)
            .interact()
            .unwrap();

        let template_path = if path_input.is_empty() {
            None
        } else {
            Some(path_input)
        };

        GitTemplate::custom(&url, branch.as_deref(), template_path.as_deref())
    };

    let source = TemplateSource::Git(git_template);

    match crate::services::project::create_project_with_source(&project_name, source).await {
        Ok(_) => println!("项目'{}'创建成功!", project_name),
        Err(e) => {
            eprintln!("创建项目时出错: {}", e);
            std::process::exit(1);
        }
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
    match crate::services::project::ai_rule::manage_ai_rule_file(&final_template, force).await {
        Ok(_) => println!("Ai Code 规则文件处理成功!"),
        Err(e) => {
            eprintln!("处理 Ai Code 规则文件时出错: {}", e);
            std::process::exit(1);
        }
    }
}
