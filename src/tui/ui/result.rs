use crate::tui::app::ResultState;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

pub(crate) fn draw_result(f: &mut Frame, area: Rect, res: &ResultState) {
    let title = match &res.result {
        Ok(_) => " 执行完成 ",
        Err(_) => " 执行失败 ",
    };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let mut lines: Vec<Line> = Vec::new();
    match &res.result {
        Ok(crate::tui::actions::TaskOutcome::Success { summary, details }) => {
            lines.push(Line::from(Span::styled(
                format!("✓ {}", summary),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )));
            for d in details {
                lines.push(Line::from(format!("  {}", d)));
            }
        }
        Err(msg) => {
            lines.push(Line::from(Span::styled(
                format!("✗ {}", msg),
                Style::default().fg(Color::Red),
            )));
        }
    }
    if res.cancelled {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "（任务曾被请求终止）",
            Style::default().fg(Color::Yellow),
        )));
    }

    f.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL).title(title)),
        chunks[0],
    );
    f.render_widget(
        Paragraph::new(Span::styled(
            "按任意键返回",
            Style::default().fg(Color::DarkGray),
        )),
        chunks[1],
    );
}
