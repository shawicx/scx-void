use std::collections::HashMap;

pub mod ai_rule;
pub mod compress;
pub mod convert;
pub mod project_init;

/// 命令所属分组（主菜单一级分区）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionGroup {
    Project,
    File,
    System,
}

pub const ALL_GROUPS: [ActionGroup; 3] = [
    ActionGroup::Project,
    ActionGroup::File,
    ActionGroup::System,
];

impl ActionGroup {
    pub fn title(&self) -> &'static str {
        match self {
            ActionGroup::Project => "项目",
            ActionGroup::File => "文件",
            ActionGroup::System => "系统",
        }
    }
}

/// 命令元数据：主菜单展示与分组
pub struct ActionMeta {
    /// 预留给 P2 的命令唯一标识/调试输出
    #[allow(dead_code)]
    pub id: &'static str,
    pub title: &'static str,
    pub description: String,
    pub group: ActionGroup,
}

/// 表单字段声明，驱动通用表单渲染与校验
#[derive(Debug, Clone)]
pub enum FieldSpec {
    Text {
        key: &'static str,
        label: String,
        placeholder: Option<String>,
        default: Option<String>,
        required: bool,
    },
    Select {
        key: &'static str,
        label: String,
        /// (值, 展示文本)；动态选项初始为空，由 resolve_options 填充
        options: Vec<(String, String)>,
        default: Option<usize>,
    },
    Toggle {
        key: &'static str,
        label: String,
        default: bool,
    },
}

impl FieldSpec {
    pub fn key(&self) -> &'static str {
        match self {
            FieldSpec::Text { key, .. }
            | FieldSpec::Select { key, .. }
            | FieldSpec::Toggle { key, .. } => key,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            FieldSpec::Text { label, .. }
            | FieldSpec::Select { label, .. }
            | FieldSpec::Toggle { label, .. } => label,
        }
    }
}

/// 表单值，与 FieldSpec 一一对应
#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    Text(String),
    Choice(String),
    Flag(bool),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FieldValues(pub HashMap<&'static str, FieldValue>);

impl FieldValues {
    pub fn text(&self, key: &str) -> Option<&str> {
        match self.0.get(key)? {
            FieldValue::Text(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn choice(&self, key: &str) -> Option<&str> {
        match self.0.get(key)? {
            FieldValue::Choice(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn flag(&self, key: &str) -> Option<bool> {
        match self.0.get(key)? {
            FieldValue::Flag(b) => Some(*b),
            _ => None,
        }
    }
}

/// 后台任务回传给 UI 的事件
#[derive(Debug, Clone)]
pub enum RuntimeEvent {
    Log(String),
    /// P2 接入压缩/下载类任务后使用
    #[allow(dead_code)]
    Progress(u8),
    Finished(Result<TaskOutcome, String>),
}

/// 任务产出
#[derive(Debug, Clone)]
pub enum TaskOutcome {
    Success {
        summary: String,
        details: Vec<String>,
    },
}

/// 任务运行上下文：进度上报 + 协作式取消 + 异步 service 桥接
///
/// log/progress/block_on 均可在阻塞线程调用（UnboundedSender::send 非阻塞）。
#[derive(Clone)]
pub struct TaskCtx {
    tx: tokio::sync::mpsc::UnboundedSender<RuntimeEvent>,
    cancel: std::sync::Arc<std::sync::atomic::AtomicBool>,
    runtime: Option<tokio::runtime::Handle>,
}

impl TaskCtx {
    pub(crate) fn new(
        tx: tokio::sync::mpsc::UnboundedSender<RuntimeEvent>,
        cancel: std::sync::Arc<std::sync::atomic::AtomicBool>,
        runtime: Option<tokio::runtime::Handle>,
    ) -> Self {
        Self {
            tx,
            cancel,
            runtime,
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test() -> Self {
        let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
        Self::new(
            tx,
            std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            tokio::runtime::Handle::try_current().ok(),
        )
    }

    /// 在阻塞线程内执行异步 service 调用（如模板下载）
    pub fn block_on<F: std::future::Future>(&self, fut: F) -> F::Output {
        self.runtime
            .as_ref()
            .expect("TaskCtx 未持有 tokio 运行时句柄")
            .block_on(fut)
    }

    pub fn log(&self, msg: impl Into<String>) {
        let _ = self.tx.send(RuntimeEvent::Log(msg.into()));
    }

    /// P2 接入压缩/下载类任务后使用
    #[allow(dead_code)]
    pub fn progress(&self, percent: u8) {
        let _ = self.tx.send(RuntimeEvent::Progress(percent));
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancel.load(std::sync::atomic::Ordering::Relaxed)
    }
}

/// 在阻塞线程执行的任务
pub trait TuiTask: Send + 'static {
    fn run(self: Box<Self>, ctx: TaskCtx) -> Result<TaskOutcome, String>;
}

/// 一个可从 TUI 执行的命令
pub trait TuiAction: Send + Sync {
    fn meta(&self) -> ActionMeta;
    fn fields(&self) -> Vec<FieldSpec>;

    /// 动态选项解析：依赖其他字段值时（如目标格式依赖输入文件），
    /// 返回 Some(options) 刷新该 Select 字段；返回 None 表示字段用静态选项。
    fn resolve_options(&self, key: &str, values: &FieldValues) -> Option<Vec<(String, String)>>;

    fn build_task(&self, values: &FieldValues) -> Box<dyn TuiTask>;
}

/// 命令注册表
pub struct ActionRegistry {
    actions: Vec<Box<dyn TuiAction>>,
}

impl ActionRegistry {
    pub fn new() -> Self {
        Self { actions: Vec::new() }
    }

    pub fn register(&mut self, action: Box<dyn TuiAction>) {
        self.actions.push(action);
    }

    /// P2 接入更多命令后的枚举入口
    #[allow(dead_code)]
    pub fn list(&self) -> &[Box<dyn TuiAction>] {
        &self.actions
    }

    pub fn into_actions(self) -> Vec<Box<dyn TuiAction>> {
        self.actions
    }

    /// 注册内置命令
    pub fn with_defaults() -> Self {
        let mut r = Self::new();
        r.register(Box::new(convert::ConvertAction));
        r.register(Box::new(ai_rule::AiRuleAction));
        r.register(Box::new(compress::CompressAction));
        r.register(Box::new(project_init::ProjectInitAction));
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_values_accessors() {
        let mut v = FieldValues::default();
        v.0.insert("file", FieldValue::Text("a.heic".into()));
        v.0.insert("format", FieldValue::Choice("png".into()));
        v.0.insert("overwrite", FieldValue::Flag(true));

        assert_eq!(v.text("file"), Some("a.heic"));
        assert_eq!(v.text("format"), None); // 类型不匹配返回 None
        assert_eq!(v.choice("format"), Some("png"));
        assert_eq!(v.flag("overwrite"), Some(true));
        assert_eq!(v.flag("missing"), None);
    }

    #[test]
    fn registry_register_and_list() {
        struct Dummy;
        impl TuiAction for Dummy {
            fn meta(&self) -> ActionMeta {
                ActionMeta {
                    id: "dummy",
                    title: "测试命令",
                    description: "用于测试".into(),
                    group: ActionGroup::File,
                }
            }
            fn fields(&self) -> Vec<FieldSpec> {
                vec![FieldSpec::Text {
                    key: "x",
                    label: "参数".into(),
                    placeholder: None,
                    default: None,
                    required: true,
                }]
            }
            fn resolve_options(
                &self,
                _key: &str,
                _values: &FieldValues,
            ) -> Option<Vec<(String, String)>> {
                None
            }
            fn build_task(&self, _values: &FieldValues) -> Box<dyn TuiTask> {
                struct Nop;
                impl TuiTask for Nop {
                    fn run(self: Box<Self>, _ctx: TaskCtx) -> Result<TaskOutcome, String> {
                        Ok(TaskOutcome::Success {
                            summary: "ok".into(),
                            details: vec![],
                        })
                    }
                }
                Box::new(Nop)
            }
        }

        let mut r = ActionRegistry::new();
        assert_eq!(r.list().len(), 0);
        r.register(Box::new(Dummy));
        assert_eq!(r.list().len(), 1);
        assert_eq!(r.list()[0].meta().title, "测试命令");
        assert_eq!(r.list()[0].fields().len(), 1);
    }
}
