use crate::tui::actions::{
    ActionRegistry, ALL_GROUPS, FieldSpec, FieldValue, FieldValues, RuntimeEvent, TaskOutcome,
    TuiAction,
};
use crate::tui::runtime::TaskHandle;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::{HashMap, VecDeque};

pub enum Message {
    /// 原始终端事件（非按键部分仅用于触发重绘）
    Input(ratatui::crossterm::event::Event),
    Key(KeyEvent),
    Tick,
    Runtime(RuntimeEvent),
}

pub enum Effect {
    None,
    StartTask(Box<dyn crate::tui::actions::TuiTask>),
}

/// 主菜单常驻状态
#[derive(Debug, Clone, Default)]
pub struct HomeState {
    pub query: String,
    pub search_mode: bool,
    pub selected: usize,
}

/// 覆盖在主菜单上的单层页面
pub enum Overlay {
    Form(FormState),
    Running(RunningState),
    Result(ResultState),
}

pub struct FormState {
    pub action_idx: usize,
    pub fields: Vec<FieldSpec>,
    /// 0..=fields.len()，fields.len() 为「提交」行
    pub focus: usize,
    pub values: FieldValues,
    pub select_pos: HashMap<&'static str, usize>,
    pub error: Option<String>,
}

impl FormState {
    pub fn new(action_idx: usize, fields: Vec<FieldSpec>) -> Self {
        let mut values = FieldValues::default();
        let mut select_pos: HashMap<&'static str, usize> = HashMap::new();
        for f in &fields {
            match f {
                FieldSpec::Text { key, default, .. } => {
                    values
                        .0
                        .insert(key, FieldValue::Text(default.clone().unwrap_or_default()));
                }
                FieldSpec::Select { key, options, default, .. } => {
                    if let Some(i) = default {
                        if let Some((v, _)) = options.get(*i) {
                            values.0.insert(key, FieldValue::Choice(v.clone()));
                            select_pos.insert(key, *i);
                        }
                    }
                }
                FieldSpec::Toggle { key, default, .. } => {
                    values.0.insert(key, FieldValue::Flag(*default));
                }
            }
        }
        Self {
            action_idx,
            fields,
            focus: 0,
            values,
            select_pos,
            error: None,
        }
    }

    /// resolve 刷新选项后，将 Choice 同步到首项
    pub fn confirm_select(&mut self) {
        if let Some(FieldSpec::Select { key, options, .. }) = self.fields.get(self.focus) {
            if let Some((v, _)) = options.first() {
                self.values.0.insert(key, FieldValue::Choice(v.clone()));
                self.select_pos.entry(key).or_insert(0);
            }
        }
    }

    /// 提交前校验：必填非空、Select 已有值
    pub fn validate(&self) -> Result<(), String> {
        for f in &self.fields {
            match f {
                FieldSpec::Text { key, label, required: true, .. } => {
                    if self.values.text(key).unwrap_or("").trim().is_empty() {
                        return Err(format!("{} 不能为空", label));
                    }
                }
                FieldSpec::Select { key, label, options, .. } => {
                    if self.values.choice(key).is_none() {
                        if options.is_empty() {
                            return Err(format!("{} 无法解析可选项（请检查依赖的输入）", label));
                        }
                        return Err(format!("请选择 {}", label));
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

pub struct RunningState {
    pub action_idx: usize,
    pub logs: VecDeque<String>,
    pub progress: Option<u8>,
    pub spinner_frame: usize,
    pub cancel_requested: bool,
    pub handle: Option<TaskHandle>,
}

impl RunningState {
    pub fn new(action_idx: usize) -> Self {
        Self {
            action_idx,
            logs: VecDeque::new(),
            progress: None,
            spinner_frame: 0,
            cancel_requested: false,
            handle: None,
        }
    }

    pub fn push_log(&mut self, msg: String) {
        self.logs.push_back(msg);
        while self.logs.len() > 200 {
            self.logs.pop_front();
        }
    }
}

pub struct ResultState {
    pub result: Result<TaskOutcome, String>,
    pub cancelled: bool,
}

pub struct App {
    pub actions: Vec<Box<dyn TuiAction>>,
    pub home: HomeState,
    pub overlay: Option<Overlay>,
    pub should_quit: bool,
    pub quit_confirm: bool,
}

impl App {
    pub fn new(registry: ActionRegistry) -> Self {
        let actions = registry.into_actions();
        Self {
            actions,
            home: HomeState::default(),
            overlay: None,
            should_quit: false,
            quit_confirm: false,
        }
    }

    /// 按分组顺序 + 搜索过滤后的可见命令下标
    pub fn visible(&self) -> Vec<usize> {
        let q = self.home.query.trim().to_lowercase();
        let mut out = Vec::new();
        for group in ALL_GROUPS {
            for (i, a) in self.actions.iter().enumerate() {
                let meta = a.meta();
                if meta.group != group {
                    continue;
                }
                if !q.is_empty()
                    && !meta.title.to_lowercase().contains(&q)
                    && !meta.description.to_lowercase().contains(&q)
                {
                    continue;
                }
                out.push(i);
            }
        }
        out
    }

    pub fn update(&mut self, msg: Message) -> Effect {
        match msg {
            Message::Input(_) => Effect::None, // 非按键事件：仅触发本轮重绘
            Message::Tick => {
                if let Some(Overlay::Running(r)) = &mut self.overlay {
                    r.spinner_frame = r.spinner_frame.wrapping_add(1);
                }
                Effect::None
            }
            Message::Runtime(ev) => self.update_runtime(ev),
            Message::Key(k) => self.update_key(k),
        }
    }

    fn update_key(&mut self, k: KeyEvent) -> Effect {
        // Ctrl-C 退出确认（所有页面一致）
        if k.modifiers.contains(KeyModifiers::CONTROL) {
            if let KeyCode::Char('c') = k.code {
                if self.quit_confirm {
                    self.should_quit = true;
                } else {
                    self.quit_confirm = true;
                }
                return Effect::None;
            }
        }
        if self.quit_confirm {
            self.quit_confirm = false; // 其他按键取消确认（该键不穿透）
            return Effect::None;
        }

        match self.overlay.take() {
            None => {
                self.update_home(k);
                Effect::None
            }
            Some(Overlay::Form(fs)) => self.update_form(k, fs),
            Some(Overlay::Running(rs)) => {
                self.update_running(k, rs);
                Effect::None
            }
            Some(Overlay::Result(_res)) => {
                self.overlay = None; // 任意键返回主菜单
                Effect::None
            }
        }
    }

    fn update_home(&mut self, k: KeyEvent) {
        if self.home.search_mode {
            match k.code {
                KeyCode::Char(c) => self.home.query.push(c),
                KeyCode::Backspace => {
                    self.home.query.pop();
                }
                KeyCode::Esc | KeyCode::Enter => self.home.search_mode = false,
                _ => {}
            }
            self.clamp_selection();
            return;
        }
        match k.code {
            KeyCode::Up => {
                self.home.selected = self.home.selected.saturating_sub(1);
            }
            KeyCode::Down => {
                let n = self.visible().len();
                if n > 0 && self.home.selected + 1 < n {
                    self.home.selected += 1;
                }
            }
            KeyCode::Char('/') => {
                self.home.search_mode = true;
            }
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Enter => {
                let vis = self.visible();
                if let Some(&idx) = vis.get(self.home.selected) {
                    let fields = self.actions[idx].fields();
                    self.overlay = Some(Overlay::Form(FormState::new(idx, fields)));
                }
            }
            _ => {}
        }
    }

    pub fn attach_task(&mut self, handle: TaskHandle) {
        if let Some(Overlay::Running(r)) = &mut self.overlay {
            r.handle = Some(handle);
        }
    }

    fn clamp_selection(&mut self) {
        let n = self.visible().len();
        if n == 0 {
            self.home.selected = 0;
        } else if self.home.selected >= n {
            self.home.selected = n - 1;
        }
    }

    fn update_form(&mut self, k: KeyEvent, mut fs: FormState) -> Effect {
        match k.code {
            KeyCode::Esc => {
                self.overlay = None;
                return Effect::None;
            }
            KeyCode::Tab => {
                fs.focus = (fs.focus + 1) % (fs.fields.len() + 1);
                self.overlay = Some(Overlay::Form(fs));
                self.refresh_options();
                return Effect::None;
            }
            KeyCode::BackTab => {
                let n = fs.fields.len() + 1;
                fs.focus = (fs.focus + n - 1) % n;
                self.overlay = Some(Overlay::Form(fs));
                self.refresh_options();
                return Effect::None;
            }
            _ => {}
        }

        let n = fs.fields.len() + 1;
        // 提交行
        if fs.focus == fs.fields.len() {
            if k.code == KeyCode::Enter {
                if let Err(msg) = fs.validate() {
                    fs.error = Some(msg);
                    self.overlay = Some(Overlay::Form(fs));
                    return Effect::None;
                }
                fs.error = None;
                let idx = fs.action_idx;
                let task = self.actions[idx].build_task(&fs.values);
                self.overlay = Some(Overlay::Running(RunningState::new(idx)));
                return Effect::StartTask(task);
            }
            self.overlay = Some(Overlay::Form(fs));
            return Effect::None;
        }

        let field_key = fs.fields[fs.focus].key();
        match (&fs.fields[fs.focus], k.code) {
            (FieldSpec::Text { .. }, KeyCode::Char(c)) => {
                if let Some(FieldValue::Text(s)) = fs.values.0.get_mut(field_key) {
                    s.push(c);
                } else {
                    fs.values
                        .0
                        .insert(field_key, FieldValue::Text(c.to_string()));
                }
            }
            (FieldSpec::Text { .. }, KeyCode::Backspace) => {
                if let Some(FieldValue::Text(s)) = fs.values.0.get_mut(field_key) {
                    s.pop();
                }
            }
            (FieldSpec::Text { .. }, KeyCode::Enter) => {
                fs.focus = (fs.focus + 1) % n;
            }
            (FieldSpec::Select { options, .. }, KeyCode::Up) => {
                let pos = fs.select_pos.get(field_key).copied().unwrap_or(0);
                if pos > 0 {
                    fs.select_pos.insert(field_key, pos - 1);
                    if let Some((v, _)) = options.get(pos - 1) {
                        fs.values
                            .0
                            .insert(field_key, FieldValue::Choice(v.clone()));
                    }
                }
            }
            (FieldSpec::Select { options, .. }, KeyCode::Down) => {
                let pos = fs.select_pos.get(field_key).copied().unwrap_or(0);
                if pos + 1 < options.len() {
                    fs.select_pos.insert(field_key, pos + 1);
                    if let Some((v, _)) = options.get(pos + 1) {
                        fs.values
                            .0
                            .insert(field_key, FieldValue::Choice(v.clone()));
                    }
                }
            }
            (FieldSpec::Select { .. }, KeyCode::Enter) => {
                fs.focus = (fs.focus + 1) % n;
            }
            (FieldSpec::Toggle { .. }, KeyCode::Char(' ')) | (FieldSpec::Toggle { .. }, KeyCode::Enter) => {
                if let Some(FieldValue::Flag(b)) = fs.values.0.get_mut(field_key) {
                    *b = !*b;
                } else {
                    fs.values.0.insert(field_key, FieldValue::Flag(true));
                }
            }
            (FieldSpec::Toggle { .. }, KeyCode::Up) => {
                fs.focus = fs.focus.saturating_sub(1);
            }
            (FieldSpec::Toggle { .. }, KeyCode::Down) => {
                fs.focus = (fs.focus + 1) % n;
            }
            _ => {}
        }
        self.overlay = Some(Overlay::Form(fs));
        self.refresh_options();
        Effect::None
    }

    /// focus 进入 options 为空的 Select 字段时解析动态选项。
    /// 同步调用（仅 32 字节文件头读取，不涉网络/子进程，见设计文档例外说明）。
    fn refresh_options(&mut self) {
        let (idx, key, values) = match &self.overlay {
            Some(Overlay::Form(fs)) => match fs.fields.get(fs.focus) {
                Some(FieldSpec::Select { key, options, .. }) if options.is_empty() => {
                    (fs.action_idx, *key, fs.values.clone())
                }
                _ => return,
            },
            _ => return,
        };
        if let Some(opts) = self.actions[idx].resolve_options(key, &values) {
            if let Some(Overlay::Form(fs)) = &mut self.overlay {
                if let Some(FieldSpec::Select { options, .. }) = fs.fields.get_mut(fs.focus) {
                    *options = opts;
                }
                fs.confirm_select();
            }
        }
    }

    fn update_running(&mut self, k: KeyEvent, mut rs: RunningState) {
        match k.code {
            KeyCode::Esc => {
                if rs.cancel_requested {
                    if let Some(h) = &rs.handle {
                        h.cancel();
                    }
                } else {
                    rs.cancel_requested = true;
                }
            }
            _ => rs.cancel_requested = false,
        }
        self.overlay = Some(Overlay::Running(rs));
    }

    fn update_runtime(&mut self, ev: RuntimeEvent) -> Effect {
        match &self.overlay {
            Some(Overlay::Running(_)) => {}
            _ => return Effect::None, // 迟到的事件（已离开执行页）丢弃
        }
        match ev {
            RuntimeEvent::Log(msg) => {
                if let Some(Overlay::Running(r)) = &mut self.overlay {
                    r.push_log(msg);
                }
            }
            RuntimeEvent::Progress(p) => {
                if let Some(Overlay::Running(r)) = &mut self.overlay {
                    r.progress = Some(p);
                }
            }
            RuntimeEvent::Finished(result) => {
                let cancelled = match &self.overlay {
                    Some(Overlay::Running(r)) => r.cancel_requested,
                    _ => false,
                };
                self.overlay = Some(Overlay::Result(ResultState { result, cancelled }));
            }
        }
        Effect::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::actions::{ActionGroup, ActionMeta, TaskCtx, TuiAction, TuiTask};

    pub fn key(code: KeyCode) -> Message {
        Message::Key(KeyEvent::new(code, KeyModifiers::NONE))
    }

    pub fn ctrl_c() -> Message {
        Message::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL))
    }

    /// 测试用命令：一个必填文本 + 一个静态 Select
    pub struct TestAction;

    impl TuiAction for TestAction {
        fn meta(&self) -> ActionMeta {
            ActionMeta {
                id: "test",
                title: "测试命令",
                description: "描述".into(),
                group: ActionGroup::File,
            }
        }
        fn fields(&self) -> Vec<FieldSpec> {
            vec![
                FieldSpec::Text {
                    key: "file",
                    label: "文件".into(),
                    placeholder: None,
                    default: None,
                    required: true,
                },
                FieldSpec::Select {
                    key: "fmt",
                    label: "格式".into(),
                    options: vec![("png".into(), "PNG".into())],
                    default: Some(0),
                },
            ]
        }
        fn resolve_options(&self, _: &str, _: &FieldValues) -> Option<Vec<(String, String)>> {
            None
        }
        fn build_task(&self, _values: &FieldValues) -> Box<dyn TuiTask> {
            struct T;
            impl TuiTask for T {
                fn run(self: Box<Self>, _: TaskCtx) -> Result<TaskOutcome, String> {
                    Ok(TaskOutcome::Success {
                        summary: "ok".into(),
                        details: vec![],
                    })
                }
            }
            Box::new(T)
        }
    }

    fn app() -> App {
        let mut r = ActionRegistry::new();
        r.register(Box::new(TestAction));
        App::new(r)
    }

    #[test]
    fn home_navigation_moves_selection() {
        let mut a = app();
        assert_eq!(a.visible().len(), 1);
        a.update(key(KeyCode::Down));
        assert_eq!(a.home.selected, 0); // 夹紧在最后一项
        a.update(key(KeyCode::Up));
        assert_eq!(a.home.selected, 0);
    }

    #[test]
    fn home_enter_opens_form_with_fields() {
        let mut a = app();
        a.update(key(KeyCode::Enter));
        match &a.overlay {
            Some(Overlay::Form(fs)) => {
                assert_eq!(fs.fields.len(), 2);
                assert_eq!(fs.focus, 0);
            }
            _ => panic!("期望 Form"),
        }
    }

    #[test]
    fn search_filters_and_esc_keeps_filter() {
        let mut a = app();
        a.update(key(KeyCode::Char('/')));
        assert!(a.home.search_mode);
        a.update(key(KeyCode::Char('x')));
        a.update(key(KeyCode::Char('y')));
        a.update(key(KeyCode::Enter)); // 退出搜索态
        assert!(!a.home.search_mode);
        assert_eq!(a.home.query, "xy");
        assert!(a.visible().is_empty()); // 无匹配
    }

    #[test]
    fn form_tab_cycles_and_esc_returns_home() {
        let mut a = app();
        a.update(key(KeyCode::Enter)); // 打开表单
        a.update(Message::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)));
        assert_eq!(form(&a).focus, 1);
        a.update(Message::Key(KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT)));
        assert_eq!(form(&a).focus, 0);
        a.update(key(KeyCode::Esc));
        assert!(a.overlay.is_none());
        assert_eq!(a.home.selected, 0); // home 状态保留
    }

    #[test]
    fn form_text_editing_and_select_change() {
        let mut a = app();
        a.update(key(KeyCode::Enter));
        a.update(key(KeyCode::Char('a')));
        a.update(key(KeyCode::Char('.')));
        a.update(key(KeyCode::Backspace));
        assert_eq!(form(&a).values.text("file"), Some("a"));
        // Tab 到 Select
        a.update(Message::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)));
        // Select 有静态 default，Choice 已初始化为 png
        assert_eq!(form(&a).values.choice("fmt"), Some("png"));
        // Down 无下一项，保持
        a.update(key(KeyCode::Down));
        assert_eq!(form(&a).values.choice("fmt"), Some("png"));
    }

    #[test]
    fn form_submit_validates_required_then_starts_task() {
        let mut a = app();
        a.update(key(KeyCode::Enter));
        // 直接连按 Tab 到提交行（fields.len()=2 → 提交行 focus=2）
        for _ in 0..2 {
            a.update(Message::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)));
        }
        let eff = a.update(key(KeyCode::Enter));
        assert!(matches!(eff, Effect::None)); // 必填为空 → 校验失败
        assert!(form(&a).error.is_some());

        // 回到 file 字段填值后再提交
        for _ in 0..2 {
            a.update(Message::Key(KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT)));
        }
        a.update(key(KeyCode::Char('x')));
        for _ in 0..2 {
            a.update(Message::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)));
        }
        let eff = a.update(key(KeyCode::Enter));
        assert!(matches!(eff, Effect::StartTask(_)));
        assert!(matches!(a.overlay, Some(Overlay::Running(_))));
    }

    fn form(a: &App) -> &FormState {
        match &a.overlay {
            Some(Overlay::Form(fs)) => fs,
            _ => panic!("期望表单"),
        }
    }

    #[test]
    fn running_first_esc_confirms_second_cancels() {
        let mut a = submitted_app();
        a.update(key(KeyCode::Esc));
        assert!(running(&a).cancel_requested);
        assert!(running(&a).handle.as_ref().unwrap().is_alive()); // 未真正取消
        a.update(key(KeyCode::Esc)); // 确认取消
        assert!(!running(&a).handle.as_ref().unwrap().is_alive()); // 已请求取消
    }

    #[test]
    fn runtime_events_build_result() {
        let mut a = submitted_app();
        a.update(Message::Runtime(RuntimeEvent::Log("步骤1".into())));
        a.update(Message::Runtime(RuntimeEvent::Progress(42)));
        assert_eq!(running(&a).progress, Some(42));
        assert_eq!(running(&a).logs.back().unwrap(), "步骤1");
        a.update(Message::Runtime(RuntimeEvent::Finished(Ok(
            TaskOutcome::Success {
                summary: "完成".into(),
                details: vec!["out.png".into()],
            },
        ))));
        match &a.overlay {
            Some(Overlay::Result(res)) => {
                assert!(res.result.is_ok());
                assert!(!res.cancelled);
            }
            _ => panic!("期望结果页"),
        }
        a.update(key(KeyCode::Enter)); // 任意键返回
        assert!(a.overlay.is_none());
    }

    fn submitted_app() -> App {
        let mut a = app();
        a.update(key(KeyCode::Enter)); // 打开表单
        a.update(key(KeyCode::Char('x'))); // 填必填
        a.update(Message::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)));
        a.update(Message::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)));
        a.update(key(KeyCode::Enter)); // 提交
        a.attach_task(TaskHandle::for_test());
        a
    }

    fn running(a: &App) -> &RunningState {
        match &a.overlay {
            Some(Overlay::Running(r)) => r,
            _ => panic!("期望执行页"),
        }
    }

    #[test]
    fn q_quits_and_ctrl_c_needs_confirm() {
        let mut a = app();
        a.update(key(KeyCode::Char('q')));
        assert!(a.should_quit);

        // 连续两次 Ctrl-C 退出
        let mut b = app();
        b.update(ctrl_c());
        assert!(!b.should_quit);
        assert!(b.quit_confirm);
        b.update(ctrl_c());
        assert!(b.should_quit);

        // 中间夹其他键则重置确认状态
        let mut c = app();
        c.update(ctrl_c());
        c.update(key(KeyCode::Down));
        assert!(!c.quit_confirm);
        c.update(ctrl_c());
        assert!(!c.should_quit); // 重新进入确认
    }
}
