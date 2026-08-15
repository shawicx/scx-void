use crate::tui::app::{App, RunningState};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, Paragraph};
use ratatui::Frame;

const SPINNER: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub(crate) fn draw_running(f: &mut Frame, area: Rect, app: &App, rs: &RunningState) {
    let title = app.actions[rs.action_idx].meta().title;
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(2)])
        .split(area);

    match rs.progress {
        Some(p) => {
            let gauge = Gauge::default()
                .block(Block::default().borders(Borders::ALL).title(format!(" {} ", title)))
                .gauge_style(Style::default().fg(Color::Cyan))
                .percent(p as u16);
            f.render_widget(gauge, chunks[0]);
        }
        None => {
            let frame_ch = SPINNER[rs.spinner_frame % SPINNER.len()];
            f.render_widget(
                Paragraph::new(Span::styled(
                    format!("{} 执行中…", frame_ch),
                    Style::default().fg(Color::Cyan),
                ))
                .block(Block::default().borders(Borders::ALL).title(format!(" {} ", title))),
                chunks[0],
            );
        }
    }

    // 日志尾部（自适应区域行数，减去上下边框）
    let rows = chunks[1].height.saturating_sub(2) as usize;
    let skip = rs.logs.len().saturating_sub(rows);
    let logs: Vec<Line> = rs
        .logs
        .iter()
        .skip(skip)
        .map(|l| Line::from(format!("› {}", l)))
        .collect();
    f.render_widget(
        Paragraph::new(logs).block(Block::default().borders(Borders::ALL).title(" 日志 ")),
        chunks[1],
    );

    let status = if rs.cancel_requested {
        Span::styled(
            "⚠ 再次按 Esc/Ctrl-C 确认终止任务，其他键继续等待",
            Style::default().fg(Color::Red),
        )
    } else {
        Span::styled("Esc 终止任务", Style::default().fg(Color::DarkGray))
    };
    f.render_widget(Paragraph::new(status), chunks[2]);
}
