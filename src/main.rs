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
    /// 环境安装命令
    Setup {
        #[command(subcommand)]
        command: cli::SetupCommands,
    },
    /// 文件格式转换
    Convert {
        /// 输入文件路径。未提供时交互式提示输入
        file: Option<String>,

        /// 目标格式（如 png）。未提供时交互式选择
        #[arg(short, long)]
        format: Option<String>,

        /// 输出文件路径。未提供则同目录同名换后缀
        #[arg(short, long)]
        output: Option<String>,

        /// 允许覆盖已存在的输出文件
        #[arg(long)]
        overwrite: bool,
    },
    /// 图片压缩为 WebP
    Compress {
        /// 输入文件路径。未提供时交互式提示输入
        file: Option<String>,

        /// 压缩质量 1-100。未提供时交互式选择预设
        #[arg(short, long)]
        quality: Option<u8>,

        /// 输出文件路径。未提供则同目录同名换 .webp
        #[arg(short, long)]
        output: Option<String>,

        /// 允许覆盖已存在的输出文件
        #[arg(long)]
        overwrite: bool,
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
        Commands::Setup { command } => {
            if let Err(e) = cli::SetupCommands::run(command).await {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Convert {
            file,
            format,
            output,
            overwrite,
        } => {
            if let Err(e) = cli::run_convert(file, format, output, overwrite).await {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Compress {
            file,
            quality,
            output,
            overwrite,
        } => {
            if let Err(e) = cli::run_compress(file, quality, output, overwrite).await {
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
