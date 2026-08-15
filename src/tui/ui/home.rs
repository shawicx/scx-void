use crate::tui::actions::{ActionGroup, FieldSpec};
use crate::tui::app::App;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::Frame;

pub(crate) fn draw_home(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // 搜索栏
            Constraint::Min(1),     // 主体
            Constraint::Length(1),  // 帮助行
        ])
        .split(f.area());

    // 搜索栏
    let (search_label, style) = if app.home.search_mode {
        (
            format!("搜索: {}_", app.home.query),
            Style::default().fg(Color::Yellow),
        )
    } else if app.home.query.is_empty() {
        (
            "按 / 搜索".to_string(),
            Style::default().fg(Color::DarkGray),
        )
    } else {
        (
            format!("搜索: {}（Esc 清除）", app.home.query),
            Style::default().fg(Color::DarkGray),
        )
    };
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(search_label, style)))
            .block(Block::default().borders(Borders::TOP).title(" scx-void ")),
        chunks[0],
    );

    // 主体：左列表右详情
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(chunks[1]);

    let visible = app.visible();
    let mut items: Vec<ListItem> = Vec::new();
    let mut last_group: Option<ActionGroup> = None;
    for &action_idx in visible.iter() {
        let meta = app.actions[action_idx].meta();
        if last_group != Some(meta.group) {
            last_group = Some(meta.group);
            items.push(ListItem::new(Line::from(Span::styled(
                format!("▸ {}", meta.group.title()),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ))));
        }
        items.push(ListItem::new(format!("  {}", meta.title)));
    }

    let list = if visible.is_empty() {
        List::new(vec![ListItem::new(Span::styled(
            "无匹配命令",
            Style::default().fg(Color::DarkGray),
        ))])
    } else {
        List::new(items)
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .highlight_symbol("❯ ")
    };
    let mut state = ListState::default();
    if !visible.is_empty() {
        // +1 跳过首个分组头（首分组头一定在第一个命令项之前）
        state.select(Some(app.home.selected + 1));
    }
    f.render_stateful_widget(list, body[0], &mut state);

    // 详情面板
    let detail = if let Some(&action_idx) = visible.get(app.home.selected) {
        let meta = app.actions[action_idx].meta();
        let mut lines = vec![
            Line::from(Span::styled(
                meta.title,
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(meta.description.clone()),
            Line::from(""),
            Line::from(Span::styled("参数:", Style::default().fg(Color::Cyan))),
        ];
        for field in app.actions[action_idx].fields() {
            let req = matches!(field, FieldSpec::Text { required: true, .. });
            lines.push(Line::from(format!(
                "  {}{}",
                field.label(),
                if req { " *" } else { "" }
            )));
        }
        lines
    } else {
        vec![Line::from(Span::styled(
            "无匹配命令",
            Style::default().fg(Color::DarkGray),
        ))]
    };
    f.render_widget(
        Paragraph::new(detail).block(Block::default().borders(Borders::LEFT)),
        body[1],
    );

    // 帮助行
    f.render_widget(
        Paragraph::new(Span::styled(
            "↑↓ 移动  / 搜索  ⏎ 执行  q 退出",
            Style::default().fg(Color::DarkGray),
        )),
        chunks[2],
    );
}
