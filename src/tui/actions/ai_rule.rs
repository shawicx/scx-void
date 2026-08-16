use crate::services::project::git::types::ProjectType;
use crate::tui::actions::{
    ActionGroup, ActionMeta, FieldSpec, FieldValues, TaskCtx, TaskOutcome, TuiAction, TuiTask,
};
use crate::utils::fs;
use std::path::PathBuf;

/// 技术栈选项（id 与 `project ai-rule -t` 一致）
const STACKS: [(&str, &str); 9] = [
    ("vue3", "Vue 3 + TypeScript + Vite"),
    ("react", "React 18 + TypeScript + Vite"),
    ("nextjs", "NextJS 14 + App Router + TypeScript"),
    ("node-cli", "Node.js + TypeScript CLI"),
    ("nestjs", "NestJS RESTful API"),
    ("tauri", "Tauri 桌面应用 (Rust + 前端)"),
    ("java", "Java (Maven/Gradle/Spring)"),
    ("rust", "Rust (Cargo)"),
    ("python", "Python (uv/poetry/pip)"),
];

pub struct AiRuleAction;

impl TuiAction for AiRuleAction {
    fn meta(&self) -> ActionMeta {
        ActionMeta {
            id: "ai-rule",
            title: "生成 AI 规则 (AGENTS.md)",
            description: "按技术栈在当前目录生成 AGENTS.md（强制覆盖时自动备份）".into(),
            group: ActionGroup::Project,
        }
    }

    fn fields(&self) -> Vec<FieldSpec> {
        vec![
            FieldSpec::Select {
                key: "type",
                label: "技术栈".into(),
                options: STACKS
                    .iter()
                    .map(|(id, desc)| (id.to_string(), format!("{} - {}", id, desc)))
                    .collect(),
                default: Some(0),
            },
            FieldSpec::Toggle {
                key: "force",
                label: "覆盖已存在的 AGENTS.md（自动备份）".into(),
                default: false,
            },
        ]
    }

    fn resolve_options(&self, _key: &str, _values: &FieldValues) -> Option<Vec<(String, String)>> {
        None // 静态选项
    }

    fn build_task(&self, values: &FieldValues) -> Box<dyn TuiTask> {
        let stack_id = values.choice("type").unwrap_or("vue3").to_string();
        // Select 的值域由 STACKS 提供，此处必然命中；兜底 vue3 仅防御异常构造
        let project_type = ProjectType::from_ai_rule_id(&stack_id).unwrap_or(ProjectType::Vue3);
        Box::new(AiRuleTask {
            stack_id,
            project_type,
            force: values.flag("force").unwrap_or(false),
            path: PathBuf::from("AGENTS.md"),
        })
    }
}

struct AiRuleTask {
    stack_id: String,
    project_type: ProjectType,
    force: bool,
    path: PathBuf,
}

impl TuiTask for AiRuleTask {
    /// 与 services::project::ai_rule::manage_ai_rule_file 行为一致的编排，
    /// 但改用 ctx.log 上报（service 层的 println! 会污染 TUI 备用屏幕）。
    fn run(self: Box<Self>, ctx: TaskCtx) -> Result<TaskOutcome, String> {
        let service = crate::services::project::ai_rule::AiRuleService::new();
        let path_str = self
            .path
            .to_str()
            .ok_or_else(|| "目标路径包含非法字符".to_string())?
            .to_string();

        if self.path.exists() && !self.force {
            return Err(format!(
                "AI 规则文件已存在: {}（勾选覆盖后重试，将自动备份）",
                self.path.display()
            ));
        }
        if self.path.exists() && self.force {
            let backup = format!("{}.backup", path_str);
            ctx.log(format!("备份现有文件 → {}", backup));
            fs::copy_file(&path_str, &backup).map_err(|e| format!("备份失败: {}", e))?;
        }

        let content = service.render(self.project_type).map_err(|e| e.to_string())?;
        ctx.log(format!("生成规则内容（技术栈: {}）", self.stack_id));
        fs::write_file(&path_str, content).map_err(|e| format!("写入失败: {}", e))?;

        Ok(TaskOutcome::Success {
            summary: format!("已生成: {}", self.path.display()),
            details: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::actions::FieldValue;

    #[test]
    fn fields_cover_params() {
        let keys: Vec<&str> = AiRuleAction.fields().iter().map(|f| f.key()).collect();
        assert_eq!(keys, vec!["type", "force"]);
    }

    #[test]
    fn build_task_maps_values() {
        let mut v = FieldValues::default();
        v.0.insert("type", FieldValue::Choice("rust".into()));
        v.0.insert("force", FieldValue::Flag(true));
        // 值映射正确性由任务行为间接验证；这里至少保证不 panic 且可执行
        let _ = AiRuleAction.build_task(&v);
    }

    #[test]
    fn registry_defaults_contain_both_actions() {
        let r = crate::tui::actions::ActionRegistry::with_defaults();
        assert_eq!(r.list().len(), 2);
        assert!(r.list().iter().any(|a| a.meta().id == "convert"));
        assert!(r.list().iter().any(|a| a.meta().id == "ai-rule"));
    }

    fn task(path: PathBuf, force: bool) -> Box<AiRuleTask> {
        Box::new(AiRuleTask {
            stack_id: "vue3".into(),
            project_type: ProjectType::Vue3,
            force,
            path,
        })
    }

    #[test]
    fn task_writes_agents_md() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("AGENTS.md");
        task(target.clone(), false).run(TaskCtx::for_test()).unwrap();
        let content = std::fs::read_to_string(&target).unwrap();
        assert!(content.contains("核心原则")); // base 段
        assert!(content.contains("Vue 3 项目规则")); // 技术栈段
    }

    #[test]
    fn task_existing_without_force_fails() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("AGENTS.md");
        std::fs::write(&target, "old").unwrap();
        let err = task(target, false).run(TaskCtx::for_test()).unwrap_err();
        assert!(err.contains("已存在"));
    }

    #[test]
    fn task_force_backups_then_overwrites() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("AGENTS.md");
        std::fs::write(&target, "old").unwrap();
        task(target.clone(), true).run(TaskCtx::for_test()).unwrap();

        assert_eq!(
            std::fs::read_to_string(dir.path().join("AGENTS.md.backup")).unwrap(),
            "old"
        );
        let content = std::fs::read_to_string(&target).unwrap();
        assert!(content.contains("核心原则"));
    }
}
