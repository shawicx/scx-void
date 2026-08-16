use crate::services::project::ai_rule::AiRuleService;
use crate::services::project::git::{downloader, registry, types::GitTemplate, validator};
use crate::tui::actions::{
    ActionGroup, ActionMeta, FieldSpec, FieldValues, TaskCtx, TaskOutcome, TuiAction, TuiTask,
};
use std::path::{Path, PathBuf};

/// 「自定义 GitHub 仓库」选项的值
const CUSTOM: &str = "__custom__";

pub struct ProjectInitAction;

impl TuiAction for ProjectInitAction {
    fn meta(&self) -> ActionMeta {
        ActionMeta {
            id: "project-init",
            title: "初始化项目",
            description: "从 GitHub 模板创建新项目（自动生成 AGENTS.md）".into(),
            group: ActionGroup::Project,
        }
    }

    fn fields(&self) -> Vec<FieldSpec> {
        let mut options: Vec<(String, String)> = registry::get_all_templates()
            .iter()
            .map(|t| (t.id.clone(), format!("{} - {}", t.display_name, t.description)))
            .collect();
        options.push((CUSTOM.to_string(), "自定义 GitHub 仓库（owner/repo）".into()));
        vec![
            FieldSpec::Text {
                key: "name",
                label: "项目名称".into(),
                placeholder: None,
                default: None,
                required: true,
            },
            FieldSpec::Select {
                key: "template",
                label: "项目模板".into(),
                options,
                default: Some(0),
            },
            FieldSpec::Text {
                key: "repo",
                label: "自定义仓库（选「自定义」时必填）".into(),
                placeholder: Some("owner/repo".into()),
                default: None,
                required: false,
            },
            FieldSpec::Text {
                key: "branch",
                label: "Git 分支（默认 main）".into(),
                placeholder: None,
                default: None,
                required: false,
            },
        ]
    }

    fn resolve_options(&self, _key: &str, _values: &FieldValues) -> Option<Vec<(String, String)>> {
        None // 静态选项（模板注册表为内存数据）
    }

    fn build_task(&self, values: &FieldValues) -> Box<dyn TuiTask> {
        let template_id = values.choice("template").unwrap_or_default().to_string();
        // 预置模板此刻解析；自定义仓库延后到任务内（需先确认 repo 已填）
        let template = if template_id == CUSTOM {
            None
        } else {
            registry::get_template_by_id(&template_id)
        };
        Box::new(ProjectInitTask {
            name: values.text("name").unwrap_or_default().trim().to_string(),
            repo: values
                .text("repo")
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from),
            branch: values
                .text("branch")
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from),
            template,
        })
    }
}

struct ProjectInitTask {
    name: String,
    repo: Option<String>,
    /// None 表示自定义仓库（用 repo + branch 构造）
    template: Option<GitTemplate>,
    branch: Option<String>,
}

impl TuiTask for ProjectInitTask {
    /// 与 project_service::create_project + cli 联动 AGENTS.md 的行为一致；
    /// 直接编排 downloader/validator（service 层 println 会污染 TUI 备用屏幕）。
    fn run(self: Box<Self>, ctx: TaskCtx) -> Result<TaskOutcome, String> {
        if self.name.is_empty() {
            return Err("项目名称不能为空".into());
        }
        if Path::new(&self.name).exists() {
            return Err(format!("项目 '{}' 已存在", self.name));
        }

        let template = match self.template {
            Some(t) => t,
            None => {
                let repo = self
                    .repo
                    .as_deref()
                    .filter(|s| !s.is_empty())
                    .ok_or_else(|| "选择「自定义 GitHub 仓库」时必须填写 owner/repo".to_string())?;
                GitTemplate::custom(repo, self.branch.as_deref(), None)
            }
        };

        validator::validate_git_template(&template).map_err(|e| e.to_string())?;

        if ctx.is_cancelled() {
            return Err("已取消".into());
        }
        ctx.log(format!("下载模板: {}", template.display_name));
        let (_temp_dir, source_dir): (tempfile::TempDir, PathBuf) = ctx
            .block_on(downloader::download_template_to_temp(
                &template,
                self.branch.as_deref(),
            ))
            .map_err(|e| e.to_string())?;

        ctx.log(format!("解压到 {}/", self.name));
        downloader::extract_template_files(&source_dir, &self.name)
            .map_err(|e| e.to_string())?;

        // AGENTS.md 生成（失败不阻断主流程，与 CLI 行为一致）
        let mut details = vec![format!("模板: {}", template.display_name)];
        let agents_path = Path::new(&self.name).join("AGENTS.md");
        match AiRuleService::new().render(template.project_type) {
            Ok(content) => match agents_path.to_str() {
                Some(p) => match crate::utils::fs::write_file(p, content) {
                    Ok(()) => details.push("已生成 AGENTS.md".into()),
                    Err(e) => ctx.log(format!("警告：AGENTS.md 生成失败: {}", e)),
                },
                None => ctx.log("警告：AGENTS.md 路径包含非法字符"),
            },
            Err(e) => ctx.log(format!("警告：AGENTS.md 生成失败: {}", e)),
        }

        Ok(TaskOutcome::Success {
            summary: format!("项目 '{}' 创建成功", self.name),
            details,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields_cover_params() {
        let keys: Vec<&str> = ProjectInitAction
            .fields()
            .iter()
            .map(|f| f.key())
            .collect();
        assert_eq!(keys, vec!["name", "template", "repo", "branch"]);
    }

    #[test]
    fn template_options_end_with_custom() {
        let fields = ProjectInitAction.fields();
        let opts = fields
            .iter()
            .find_map(|f| match f {
                FieldSpec::Select { key: "template", options, .. } => Some(options.clone()),
                _ => None,
            })
            .unwrap();
        assert_eq!(opts.len(), registry::get_all_templates().len() + 1);
        assert_eq!(opts.last().unwrap().0, CUSTOM);
    }

    fn task(name: &str, repo: Option<&str>, template: Option<GitTemplate>) -> Box<ProjectInitTask> {
        Box::new(ProjectInitTask {
            name: name.into(),
            repo: repo.map(String::from),
            template,
            branch: None,
        })
    }

    #[test]
    fn task_name_empty_fails() {
        let err = task("", None, None).run(TaskCtx::for_test()).unwrap_err();
        assert!(err.contains("项目名称不能为空"));
    }

    #[test]
    fn task_dir_exists_fails() {
        let dir = tempfile::tempdir().unwrap();
        let name = dir.path().to_string_lossy().into_owned();
        let err = task(&name, None, None).run(TaskCtx::for_test()).unwrap_err();
        assert!(err.contains("已存在"));
    }

    #[test]
    fn task_custom_without_repo_fails() {
        let err = task("unique-name-xyz", None, None)
            .run(TaskCtx::for_test())
            .unwrap_err();
        assert!(err.contains("必须填写"));
    }

    #[test]
    fn task_invalid_repo_format_fails() {
        // 自定义仓库但 repo 非法（非 owner/repo），validator 拦截，不会发起网络请求
        let err = task("unique-name-xyz", Some("not a github url"), None)
            .run(TaskCtx::for_test())
            .unwrap_err();
        assert!(err.contains("GitHub URL"));
    }

    #[test]
    fn registry_defaults_contain_project_init() {
        let r = crate::tui::actions::ActionRegistry::with_defaults();
        assert!(r.list().iter().any(|a| a.meta().id == "project-init"));
    }
}
