use crate::services::system::ShutdownService;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum SystemCommands {
    /// 关闭系统
    Shutdown {
        #[arg(long, help = "关机计时器（以秒为单位）", default_value_t = 0)]
        timer: u64,
    },
}

impl SystemCommands {
    pub async fn run(command: SystemCommands) -> Result<(), Box<dyn std::error::Error>> {
        match command {
            SystemCommands::Shutdown { timer } => shutdown_system(timer).await,
        }
    }
}

async fn shutdown_system(timer: u64) -> Result<(), Box<dyn std::error::Error>> {
    println!("系统将在{}秒后关闭...", timer);
    ShutdownService::shutdown_in(timer)
}
