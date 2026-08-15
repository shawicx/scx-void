pub mod actions;
pub mod app;
pub mod runtime;
pub mod ui;

use app::{App, Effect, Message};
use ratatui::crossterm::event::{Event, KeyEventKind};
use std::io::IsTerminal;
use std::time::Duration;

/// 进入全屏 TUI 界面。
///
/// 非 TTY 环境（管道/CI）直接返回错误，避免挂死。
/// 终端初始化/恢复由 ratatui::init/restore 处理（含 panic 时恢复终端）。
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    if !std::io::stdout().is_terminal() {
        return Err("TUI 需要交互式终端（当前 stdout 不是 TTY）".into());
    }

    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal).await;
    ratatui::restore();
    result
}

async fn run_app(terminal: &mut ratatui::DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new(actions::ActionRegistry::with_defaults());

    // 输入线程：crossterm 阻塞读 → 统一消息通道（send 失败即主循环已退出）
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();
    let input_tx = tx.clone();
    std::thread::spawn(move || loop {
        match ratatui::crossterm::event::read() {
            Ok(ev) => {
                if input_tx.send(Message::Input(ev)).is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    });

    let mut ticker = tokio::time::interval(Duration::from_millis(100));
    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        tokio::select! {
            msg = rx.recv() => {
                let Some(msg) = msg else { break };
                match msg {
                    Message::Input(Event::Key(k)) => {
                        if k.kind == KeyEventKind::Press {
                            if let Effect::StartTask(task) = app.update(Message::Key(k)) {
                                let (handle, mut task_rx) = runtime::spawn(task);
                                app.attach_task(handle);
                                // 任务事件转发进统一消息通道
                                let forward_tx = tx.clone();
                                tokio::spawn(async move {
                                    while let Some(ev) = task_rx.recv().await {
                                        if forward_tx.send(Message::Runtime(ev)).is_err() {
                                            break;
                                        }
                                    }
                                });
                            }
                        }
                    }
                    other => {
                        let _ = app.update(other); // 非按键 Input / Key(测试) / Tick / Runtime
                    }
                }
            }
            _ = ticker.tick() => {
                app.update(Message::Tick);
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}
