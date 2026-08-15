use crate::tui::actions::{FieldSpec, FieldValue};
use crate::tui::app::FormState;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

pub(crate) fn draw_form(f: &mut Frame, area: Rect, fs: &FormState, title: &str) {
    let popup = centered_rect(area, 70, 70);
    f.render_widget(Clear, popup);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1), Constraint::Length(1)])
        .split(popup);

    let n = fs.fields.len();
    let mut lines: Vec<Line> = Vec::new();
    for (i, field) in fs.fields.iter().enumerate() {
        let focused = i == fs.focus;
        let base = if focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let line = match field {
            FieldSpec::Text { placeholder, .. } => {
                let raw = fs.values.0.get(field.key());
                let has_value = matches!(raw, Some(FieldValue::Text(s)) if !s.is_empty());
                let shown = match raw {
                    Some(FieldValue::Text(s)) if !s.is_empty() => {
                        if focused {
                            format!("{}▏", s)
                        } else {
                            s.clone()
                        }
                    }
                    _ => placeholder.clone().unwrap_or_default(),
                };
                Line::from(vec![
                    Span::styled(format!("  {}: ", field.label()), base),
                    Span::styled(
                        shown,
                        if has_value {
                            base
                        } else {
                            Style::default().fg(Color::DarkGray)
                        },
                    ),
                ])
            }
            FieldSpec::Select { options, .. } => {
                let pos = fs.select_pos.get(field.key()).copied().unwrap_or(0);
                let current = options
                    .get(pos)
                    .map(|(_, d)| d.clone())
                    .unwrap_or_else(|| "（待解析…聚焦后自动检测）".into());
                let mut spans = vec![
                    Span::styled(format!("  {}: ", field.label()), base),
                    Span::styled(format!("‹ {} ›", current), base),
                ];
                if focused && !options.is_empty() {
                    spans.push(Span::styled("  ↑↓ 切换", Style::default().fg(Color::DarkGray)));
                }
                Line::from(spans)
            }
            FieldSpec::Toggle { .. } => {
                let on = fs.values.flag(field.key()).unwrap_or(false);
                Line::from(vec![
                    Span::styled(format!("  {}: ", field.label()), base),
                    Span::styled(if on { "[✓] 是" } else { "[ ] 否" }, base),
                    Span::styled("  Space 切换", Style::default().fg(Color::DarkGray)),
                ])
            }
        };
        lines.push(line);
    }
    // 提交行
    let submit_focused = fs.focus == n;
    lines.push(Line::from(Span::styled(
        "  「 提交 ⏎ 」",
        if submit_focused {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Green)
        },
    )));

    f.render_widget(
        Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} ", title)),
        ),
        chunks[0],
    );

    let err = fs.error.clone().unwrap_or_default();
    f.render_widget(
        Paragraph::new(Span::styled(
            if err.is_empty() {
                " ".to_string()
            } else {
                format!("✗ {}", err)
            },
            Style::default().fg(Color::Red),
        )),
        chunks[1],
    );
    f.render_widget(
        Paragraph::new(Span::styled(
            "Tab 切换  ↑↓ 选择/移动  Space 切换  ⏎ 提交  Esc 返回",
            Style::default().fg(Color::DarkGray),
        )),
        chunks[2],
    );
}

fn centered_rect(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
