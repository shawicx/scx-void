use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::services::AudioService;
use crate::errors::ScxVoidError;

#[derive(Parser, Debug)]
pub struct AudioCommands {
    #[command(subcommand)]
    pub command: AudioSubCommands,
}

#[derive(Subcommand, Debug)]
pub enum AudioSubCommands {
    /// 转录音频文件为文本
    Transcribe {
        /// 音频文件路径 (支持 M4A, AAC, MP4 格式)
        #[arg(help = "音频文件路径")]
        file: PathBuf,
        /// 指定语言代码 (如: en, zh, auto)，默认为自动检测
        #[arg(long, short, help = "语言代码")]
        lang: Option<String>,
        /// 指定 Whisper 模型名称或路径 (如: tiny, base, medium, 或完整路径)
        #[arg(long, short, help = "模型名称或路径")]
        model: Option<String>,
        /// 输出文件路径，默认输出到控制台
        #[arg(long, short, help = "输出文件路径")]
        output: Option<PathBuf>,
        /// 跳过前 N 秒 (用于跳过风噪段)
        #[arg(long, default_value = "0", help = "跳过前 N 秒")]
        skip_seconds: u64,
        /// 结束时间点 (秒)，默认处理到音频结尾
        #[arg(long, help = "结束时间点 (秒)")]
        end_time: Option<u64>,
        /// 温度参数 (0.0-1.0)，控制输出随机性
        #[arg(long, default_value = "0.3", help = "温度参数 (0.0-1.0)")]
        temperature: f32,
        /// Beam search 大小，提高准确性
        #[arg(long, default_value = "5", help = "Beam search 大小")]
        beam_size: i32,
        /// 静音检测阈值，过滤风声和静音段
        #[arg(long, default_value = "0.6", help = "静音检测阈值")]
        no_speech_threshold: f32,
    },
    /// 转录音频文件并生成带时间戳的文本
    TranscribeWithTimestamps {
        /// 音频文件路径 (支持 M4A, AAC, MP4 格式)
        #[arg(help = "音频文件路径")]
        file: PathBuf,
        /// 指定语言代码 (如: en, zh, auto)，默认为自动检测
        #[arg(long, short, help = "语言代码")]
        lang: Option<String>,
        /// 指定 Whisper 模型名称或路径 (如: tiny, base, medium, 或完整路径)
        #[arg(long, short, help = "模型名称或路径")]
        model: Option<String>,
        /// 输出文件路径，默认输出到控制台
        #[arg(long, short, help = "输出文件路径")]
        output: Option<PathBuf>,
        /// 跳过前 N 秒 (用于跳过风噪段)
        #[arg(long, default_value = "0", help = "跳过前 N 秒")]
        skip_seconds: u64,
        /// 结束时间点 (秒)，默认处理到音频结尾
        #[arg(long, help = "结束时间点 (秒)")]
        end_time: Option<u64>,
        /// 温度参数 (0.0-1.0)，控制输出随机性
        #[arg(long, default_value = "0.3", help = "温度参数 (0.0-1.0)")]
        temperature: f32,
        /// Beam search 大小，提高准确性
        #[arg(long, default_value = "5", help = "Beam search 大小")]
        beam_size: i32,
        /// 静音检测阈值，过滤风声和静音段
        #[arg(long, default_value = "0.6", help = "静音检测阈值")]
        no_speech_threshold: f32,
    },
    /// 下载 Whisper 模型
    DownloadModel {
        /// 模型大小 (tiny, base, small, medium, large)
        #[arg(help = "模型大小")]
        size: String,
    },
    /// 列出可用的 Whisper 模型
    ListModels,
}

impl AudioCommands {
    pub async fn run(self) -> Result<(), ScxVoidError> {
        let mut audio_service = AudioService::new()
            .map_err(|e| ScxVoidError::GeneralError(e))?;

        match self.command {
            AudioSubCommands::Transcribe { file, lang, model, output, skip_seconds, end_time, temperature, beam_size, no_speech_threshold } => {
                audio_service.transcribe_file_with_advanced_params(file, lang, model, output, skip_seconds, end_time, temperature, beam_size, no_speech_threshold).await
                    .map_err(|e| ScxVoidError::TranscriptionError(e))?;
            }
            AudioSubCommands::TranscribeWithTimestamps { file, lang, model, output, skip_seconds, end_time, temperature, beam_size, no_speech_threshold } => {
                audio_service.transcribe_with_timestamps_and_advanced_params(file, lang, model, output, skip_seconds, end_time, temperature, beam_size, no_speech_threshold).await
                    .map_err(|e| ScxVoidError::TranscriptionError(e))?;
            }
            AudioSubCommands::DownloadModel { size } => {
                audio_service.download_model(&size).await
                    .map_err(|e| ScxVoidError::ModelDownloadError(e))?;
            }
            AudioSubCommands::ListModels => {
                audio_service.list_models()
                    .map_err(|e| ScxVoidError::GeneralError(e))?;
            }
        }

        Ok(())
    }
}