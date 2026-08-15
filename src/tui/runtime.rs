use super::actions::{RuntimeEvent, TaskCtx, TuiTask};

#[derive(Clone)]
pub struct TaskHandle {
    cancel: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl TaskHandle {
    pub fn cancel(&self) {
        self.cancel.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// 尚未请求取消（仅测试使用；UI 状态由 RunningState.cancel_requested 表达）
    #[cfg(test)]
    pub fn is_alive(&self) -> bool {
        !self.cancel.load(std::sync::atomic::Ordering::Relaxed)
    }

    #[cfg(test)]
    pub fn for_test() -> Self {
        Self {
            cancel: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }
}

/// 在阻塞线程执行任务，返回取消句柄与事件流接收端。
///
/// 任务结束后（无论成败/取消）必发一个 Finished 事件。
pub fn spawn(task: Box<dyn TuiTask>) -> (TaskHandle, tokio::sync::mpsc::UnboundedReceiver<RuntimeEvent>) {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let cancel = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let ctx = TaskCtx::new(tx.clone(), cancel.clone());
    let forwarder = tx;
    tokio::spawn(async move {
        let result = tokio::task::spawn_blocking(move || task.run(ctx))
            .await
            .map_err(|e| format!("任务线程崩溃: {}", e))
            .and_then(|r| r);
        let _ = forwarder.send(RuntimeEvent::Finished(result));
    });
    (TaskHandle { cancel }, rx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::actions::TaskOutcome;

    struct FakeTask {
        events: Vec<RuntimeEvent>,
    }

    impl TuiTask for FakeTask {
        fn run(self: Box<Self>, ctx: TaskCtx) -> Result<TaskOutcome, String> {
            for ev in self.events {
                match ev {
                    RuntimeEvent::Log(m) => ctx.log(m),
                    RuntimeEvent::Progress(p) => ctx.progress(p),
                    RuntimeEvent::Finished(_) => unreachable!(),
                }
            }
            Ok(TaskOutcome::Success {
                summary: "done".into(),
                details: vec![],
            })
        }
    }

    #[tokio::test]
    async fn events_arrive_in_order_then_finished() {
        let (_handle, mut rx) = spawn(Box::new(FakeTask {
            events: vec![RuntimeEvent::Log("a".into()), RuntimeEvent::Progress(50)],
        }));
        let mut got = vec![];
        while let Some(ev) = rx.recv().await {
            let is_finished = matches!(ev, RuntimeEvent::Finished(Ok(_)));
            got.push(format!("{:?}", ev));
            if is_finished {
                break;
            }
        }
        assert!(got[0].contains("Log"));
        assert!(got[1].contains("Progress"));
        assert!(got[2].contains("Success"));
    }

    #[tokio::test]
    async fn cancel_flag_visible_to_task() {
        struct CancellingTask;
        impl TuiTask for CancellingTask {
            fn run(self: Box<Self>, ctx: TaskCtx) -> Result<TaskOutcome, String> {
                while !ctx.is_cancelled() {
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
                Err("已取消".into())
            }
        }
        let (handle, mut rx) = spawn(Box::new(CancellingTask));
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        handle.cancel();
        let last = rx.recv().await;
        assert!(matches!(last, Some(RuntimeEvent::Finished(Err(msg))) if msg.contains("取消")));
    }
}
