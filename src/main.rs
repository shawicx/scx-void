use clap::{Parser, Subcommand};

mod cli;
mod errors;
mod platform;
mod services;
mod utils;

#[derive(Parser)]
#[command(name = "scx-void")]
#[command(version = "0.1.0")]
#[command(about = "用于项目管理和系统操作的多功能命令行工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 项目相关命令
    Project {
        #[command(subcommand)]
        command: cli::ProjectCommands,
    },
    /// 系统相关命令
    System {
        #[command(subcommand)]
        command: cli::SystemCommands,
    },
    #[cfg(feature = "audio")]
    /// 音频转录相关命令
    Audio {
        #[command(subcommand)]
        command: cli::AudioSubCommands,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Project { command } => {
            cli::ProjectCommands::run(command).await;
        }
        Commands::System { command } => {
            if let Err(e) = cli::SystemCommands::run(command).await {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        #[cfg(feature = "audio")]
        Commands::Audio { command } => {
            let audio_command = cli::AudioCommands { command };
            if let Err(e) = audio_command.run().await {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
