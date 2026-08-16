use crate::tui::app::App;
use ratatui::Frame;

mod form;
mod home;
mod result;
mod running;

pub fn draw(f: &mut Frame, app: &App) {
    match &app.overlay {
        Some(crate::tui::app::Overlay::Form(fs)) => {
            let title = app.actions[fs.action_idx].meta().title;
            form::draw_form(f, f.area(), fs, title);
        }
        Some(crate::tui::app::Overlay::Running(rs)) => {
            running::draw_running(f, f.area(), app, rs);
        }
        Some(crate::tui::app::Overlay::Result(res)) => {
            result::draw_result(f, f.area(), res);
        }
        None => home::draw_home(f, app),
    }
}

#[cfg(test)]
pub(crate) fn buffer_to_string(buffer: &ratatui::buffer::Buffer) -> String {
    use ratatui::buffer::CellWidth;

    let width = buffer.area().width as usize;
    let mut out = String::new();
    let mut skip = 0usize;
    for (i, cell) in buffer.content().iter().enumerate() {
        if i > 0 && i % width == 0 {
            out.push('\n');
            skip = 0;
        }
        if skip == 0 {
            out.push_str(cell.symbol());
        }
        // 与 ratatui Debug 实现一致：按符号宽度跳过续格（CJK 双宽字符）
        skip = skip.max(cell.cell_width() as usize).saturating_sub(1);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::actions::{
        ActionGroup, ActionMeta, ActionRegistry, FieldSpec, FieldValues, TaskCtx, TaskOutcome,
        TuiAction, TuiTask,
    };
    use crate::tui::app::{Message, Overlay};
    use crate::tui::runtime::TaskHandle;
    use ratatui::backend::TestBackend;
    use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    pub struct UiAction;

    impl TuiAction for UiAction {
        fn meta(&self) -> ActionMeta {
            ActionMeta {
                id: "ui",
                title: "格式转换",
                description: "检测源格式并转换".into(),
                group: ActionGroup::File,
            }
        }
        fn fields(&self) -> Vec<FieldSpec> {
            vec![
                FieldSpec::Text {
                    key: "file",
                    label: "输入文件路径".into(),
                    placeholder: Some("a.heic".into()),
                    default: None,
                    required: true,
                },
                FieldSpec::Toggle {
                    key: "overwrite",
                    label: "覆盖已存在文件".into(),
                    default: false,
                },
            ]
        }
        fn resolve_options(&self, _: &str, _: &FieldValues) -> Option<Vec<(String, String)>> {
            None
        }
        fn build_task(&self, _: &FieldValues) -> Box<dyn TuiTask> {
            struct T;
            impl TuiTask for T {
                fn run(self: Box<Self>, _: TaskCtx) -> Result<TaskOutcome, String> {
                    Ok(TaskOutcome::Success {
                        summary: String::new(),
                        details: vec![],
                    })
                }
            }
            Box::new(T)
        }
    }

    fn render(app: &App, width: u16, height: u16) -> String {
        let mut terminal = ratatui::Terminal::new(TestBackend::new(width, height)).unwrap();
        terminal.draw(|f| draw(f, app)).unwrap();
        buffer_to_string(terminal.backend().buffer())
    }

    fn app() -> App {
        let mut r = ActionRegistry::new();
        r.register(Box::new(UiAction));
        App::new(r)
    }

    fn key(code: KeyCode) -> Message {
        Message::Key(KeyEvent::new(code, KeyModifiers::NONE))
    }

    fn tab() -> Message {
        Message::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE))
    }

    #[test]
    fn home_renders_all_default_actions_grouped() {
        let app = App::new(crate::tui::actions::ActionRegistry::with_defaults());
        let text = render(&app, 100, 24);
        // 项目组在前，文件组在后，各 2 个命令
        assert!(text.contains("项目"));
        assert!(text.contains("初始化项目"));
        assert!(text.contains("生成 AI 规则 (AGENTS.md)"));
        assert!(text.contains("文件"));
        assert!(text.contains("格式转换"));
        assert!(text.contains("图片压缩 (WebP)"));
        assert!(text.find("项目").unwrap() < text.find("初始化项目").unwrap());
    }

    #[test]
    fn home_shows_group_title_action_and_help() {
        let text = render(&app(), 80, 24);
        assert!(text.contains("scx-void"));
        assert!(text.contains("文件"));
        assert!(text.contains("格式转换"));
        assert!(text.contains("检测源格式并转换"));
        assert!(text.contains("/ 搜索"));
    }

    #[test]
    fn home_search_hides_unmatched() {
        let mut a = app();
        a.update(key(KeyCode::Char('/')));
        a.update(key(KeyCode::Char('z')));
        let text = render(&a, 80, 24);
        assert!(!text.contains("格式转换"));
        assert!(text.contains("无匹配命令"));
    }

    #[test]
    fn form_shows_labels_placeholder_and_help() {
        let mut a = app();
        a.update(key(KeyCode::Enter));
        let text = render(&a, 80, 24);
        assert!(text.contains("格式转换"));
        assert!(text.contains("输入文件路径"));
        assert!(text.contains("a.heic"));
        assert!(text.contains("覆盖已存在文件"));
        assert!(text.contains("Tab 切换"));
        assert!(text.contains("提交"));
    }

    #[test]
    fn form_shows_error_after_failed_submit() {
        let mut a = app();
        a.update(key(KeyCode::Enter));
        a.update(tab());
        a.update(tab());
        a.update(key(KeyCode::Enter)); // 校验失败
        let text = render(&a, 80, 24);
        assert!(text.contains("不能为空"));
    }

    fn running_app() -> App {
        let mut a = app();
        a.update(key(KeyCode::Enter));
        a.update(key(KeyCode::Char('x')));
        a.update(tab());
        a.update(tab());
        a.update(key(KeyCode::Enter));
        a.attach_task(TaskHandle::for_test());
        assert!(matches!(a.overlay, Some(Overlay::Running(_))));
        a
    }

    #[test]
    fn running_shows_logs_progress_and_cancel_hint() {
        let mut a = running_app();
        a.update(Message::Runtime(crate::tui::actions::RuntimeEvent::Log(
            "检测到格式: HEIC".into(),
        )));
        a.update(Message::Runtime(crate::tui::actions::RuntimeEvent::Progress(68)));
        let text = render(&a, 80, 24);
        assert!(text.contains("68%"));
        assert!(text.contains("检测到格式: HEIC"));
        assert!(text.contains("Esc 终止"));
    }

    #[test]
    fn running_cancel_confirm_line() {
        let mut a = running_app();
        a.update(key(KeyCode::Esc));
        let text = render(&a, 80, 24);
        assert!(text.contains("再次按"));
    }

    #[test]
    fn result_success_and_error() {
        let mut a = running_app();
        a.update(Message::Runtime(crate::tui::actions::RuntimeEvent::Finished(Ok(
            TaskOutcome::Success {
                summary: "已生成: out.png".into(),
                details: vec![],
            },
        ))));
        let text = render(&a, 80, 24);
        assert!(text.contains("✓"));
        assert!(text.contains("已生成: out.png"));
        assert!(text.contains("任意键"));

        let mut b = running_app();
        b.update(Message::Runtime(crate::tui::actions::RuntimeEvent::Finished(Err(
            "文件不存在: x.heic".into(),
        ))));
        let text = render(&b, 80, 24);
        assert!(text.contains("✗"));
        assert!(text.contains("文件不存在: x.heic"));
    }
}

