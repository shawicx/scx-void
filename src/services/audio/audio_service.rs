use std::path::PathBuf;
use indicatif::{ProgressBar, ProgressStyle};
use crate::services::audio::{AudioDecoder, WhisperTranscriber, ModelManager};

pub struct AudioService {
    decoder: AudioDecoder,
    model_manager: ModelManager,
}

impl AudioService {
    pub fn new() -> Result<Self, String> {
        let model_manager = ModelManager::new()?;
        Ok(Self {
            decoder: AudioDecoder::new(),
            model_manager,
        })
    }

    pub async fn transcribe_file(
        &mut self,
        file_path: PathBuf,
        language: Option<String>,
        model: Option<String>,
        output: Option<PathBuf>,
    ) -> Result<(), String> {
        // 验证输入文件
        if !file_path.exists() {
            return Err(format!("音频文件不存在: {:?}", file_path));
        }

        // 确定使用的模型
        let model_path = match model {
            Some(model_input) => {
                // 智能查找模型：支持模型名称或完整路径
                self.model_manager.find_model_by_name_or_path(&model_input)
                    .ok_or_else(|| {
                        format!("找不到模型 '{}'。可用选项: tiny, base, small, medium, large，或者指定完整路径", model_input)
                    })?
            }
            None => {
                // 使用默认模型 (base)
                self.model_manager.get_model_path("base")
                    .ok_or("未找到默认模型 (base)。请先下载模型或使用 --model 参数指定模型")?
            }
        };

        // 验证模型文件
        self.model_manager.validate_model(&model_path)?;

        println!("正在解码音频文件: {:?}", file_path);

        // 显示解码进度
        let decode_progress = ProgressBar::new_spinner();
        decode_progress.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
        );
        decode_progress.set_message("解码音频中...");

        // 解码音频
        let pcm_data = self.decoder.decode_to_pcm(&file_path)
            .map_err(|e| format!("音频解码失败: {}", e))?;

        decode_progress.finish_with_message("音频解码完成");
        println!("音频解码完成，样本数: {}", pcm_data.len());

        if pcm_data.is_empty() {
            return Err("解码后的音频数据为空".to_string());
        }

        println!("正在加载 Whisper 模型...");

        // 创建转录器
        let mut transcriber = WhisperTranscriber::new(&model_path.to_string_lossy())
            .map_err(|e| format!("无法初始化转录器: {}", e))?;

        println!("开始转录...");

        // 显示转录进度
        let transcribe_progress = ProgressBar::new_spinner();
        transcribe_progress.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.blue} {msg}")
                .unwrap()
        );
        transcribe_progress.set_message("转录中，请稍候...");

        // 执行转录
        let language_ref = language.as_deref();
        let result = transcriber.transcribe_to_text(&pcm_data, language_ref)
            .map_err(|e| format!("转录失败: {}", e))?;

        transcribe_progress.finish_with_message("转录完成");

        // 输出结果
        match output {
            Some(output_path) => {
                std::fs::write(&output_path, &result)
                    .map_err(|e| format!("无法写入输出文件: {}", e))?;
                println!("转录结果已保存到: {:?}", output_path);
            }
            None => {
                println!("\n=== 转录结果 ===");
                println!("{}", result);
                println!("==================");
            }
        }

        Ok(())
    }

    pub async fn transcribe_with_timestamps(
        &mut self,
        file_path: PathBuf,
        language: Option<String>,
        model: Option<String>,
        output: Option<PathBuf>,
    ) -> Result<(), String> {
        // 验证输入文件
        if !file_path.exists() {
            return Err(format!("音频文件不存在: {:?}", file_path));
        }

        // 确定使用的模型
        let model_path = match model {
            Some(model_input) => {
                // 智能查找模型：支持模型名称或完整路径
                self.model_manager.find_model_by_name_or_path(&model_input)
                    .ok_or_else(|| {
                        format!("找不到模型 '{}'。可用选项: tiny, base, small, medium, large，或者指定完整路径", model_input)
                    })?
            }
            None => {
                // 使用默认模型 (base)
                self.model_manager.get_model_path("base")
                    .ok_or("未找到默认模型 (base)。请先下载模型或使用 --model 参数指定模型")?
            }
        };

        // 验证模型文件
        self.model_manager.validate_model(&model_path)?;

        println!("正在解码音频文件: {:?}", file_path);

        // 解码音频
        let pcm_data = self.decoder.decode_to_pcm(&file_path)
            .map_err(|e| format!("音频解码失败: {}", e))?;

        if pcm_data.is_empty() {
            return Err("解码后的音频数据为空".to_string());
        }

        println!("正在加载 Whisper 模型...");

        // 创建转录器
        let mut transcriber = WhisperTranscriber::new(&model_path.to_string_lossy())
            .map_err(|e| format!("无法初始化转录器: {}", e))?;

        println!("开始转录（带时间戳）...");

        // 执行转录获取分段
        let language_ref = language.as_deref();
        let segments = transcriber.transcribe(&pcm_data, language_ref)
            .map_err(|e| format!("转录失败: {}", e))?;

        // 生成 SRT 格式输出
        let mut srt_content = String::new();
        for (index, segment) in segments.iter().enumerate() {
            srt_content.push_str(&segment.to_srt_format(index + 1));
            srt_content.push('\n');
        }

        // 输出结果
        match output {
            Some(output_path) => {
                std::fs::write(&output_path, srt_content)
                    .map_err(|e| format!("无法写入输出文件: {}", e))?;
                println!("带时间戳的转录结果已保存到: {:?}", output_path);
            }
            None => {
                println!("\n=== 转录结果（带时间戳）===");
                println!("{}", srt_content);
                println!("============================");
            }
        }

        Ok(())
    }

    pub async fn transcribe_file_with_filter(
        &mut self,
        file_path: PathBuf,
        language: Option<String>,
        model: Option<String>,
        output: Option<PathBuf>,
        skip_seconds: u64,
        end_time: Option<u64>,
    ) -> Result<(), String> {
        // 验证输入文件
        if !file_path.exists() {
            return Err(format!("音频文件不存在: {:?}", file_path));
        }

        // 确定使用的模型
        let model_path = match model {
            Some(model_input) => {
                // 智能查找模型：支持模型名称或完整路径
                self.model_manager.find_model_by_name_or_path(&model_input)
                    .ok_or_else(|| {
                        format!("找不到模型 '{}'。可用选项: tiny, base, small, medium, large，或者指定完整路径", model_input)
                    })?
            }
            None => {
                // 使用默认模型 (base)
                self.model_manager.get_model_path("base")
                    .ok_or("未找到默认模型 (base)。请先下载模型或使用 --model 参数指定模型")?
            }
        };

        // 验证模型文件
        self.model_manager.validate_model(&model_path)?;

        println!("正在解码音频文件: {:?}", file_path);

        // 显示解码进度
        let decode_progress = ProgressBar::new_spinner();
        decode_progress.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
        );
        decode_progress.set_message("解码音频中...");

        // 解码音频
        let pcm_data = self.decoder.decode_to_pcm(&file_path)
            .map_err(|e| format!("音频解码失败: {}", e))?;

        decode_progress.finish_with_message("音频解码完成");
        println!("音频解码完成，样本数: {}", pcm_data.len());

        if pcm_data.is_empty() {
            return Err("解码后的音频数据为空".to_string());
        }

        // 应用时间过滤
        let filtered_pcm_data = self.apply_time_filter(&pcm_data, skip_seconds, end_time)?;
        println!("时间过滤后样本数: {} (跳过前 {} 秒)", filtered_pcm_data.len(), skip_seconds);

        if filtered_pcm_data.is_empty() {
            return Err("时间过滤后音频数据为空".to_string());
        }

        println!("正在加载 Whisper 模型...");

        // 创建转录器
        let mut transcriber = WhisperTranscriber::new(&model_path.to_string_lossy())
            .map_err(|e| format!("无法初始化转录器: {}", e))?;

        println!("开始转录...");

        // 显示转录进度
        let transcribe_progress = ProgressBar::new_spinner();
        transcribe_progress.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.blue} {msg}")
                .unwrap()
        );
        transcribe_progress.set_message("转录中，请稍候...");

        // 执行转录
        let language_ref = language.as_deref();
        let result = transcriber.transcribe_to_text(&filtered_pcm_data, language_ref)
            .map_err(|e| format!("转录失败: {}", e))?;

        transcribe_progress.finish_with_message("转录完成");

        // 输出结果
        match output {
            Some(output_path) => {
                std::fs::write(&output_path, &result)
                    .map_err(|e| format!("无法写入输出文件: {}", e))?;
                println!("转录结果已保存到: {:?}", output_path);
            }
            None => {
                println!("\n=== 转录结果 ===");
                println!("{}", result);
                println!("==================");
            }
        }

        Ok(())
    }

    pub async fn transcribe_with_timestamps_and_filter(
        &mut self,
        file_path: PathBuf,
        language: Option<String>,
        model: Option<String>,
        output: Option<PathBuf>,
        skip_seconds: u64,
        end_time: Option<u64>,
    ) -> Result<(), String> {
        // 验证输入文件
        if !file_path.exists() {
            return Err(format!("音频文件不存在: {:?}", file_path));
        }

        // 确定使用的模型
        let model_path = match model {
            Some(model_input) => {
                // 智能查找模型：支持模型名称或完整路径
                self.model_manager.find_model_by_name_or_path(&model_input)
                    .ok_or_else(|| {
                        format!("找不到模型 '{}'。可用选项: tiny, base, small, medium, large，或者指定完整路径", model_input)
                    })?
            }
            None => {
                // 使用默认模型 (base)
                self.model_manager.get_model_path("base")
                    .ok_or("未找到默认模型 (base)。请先下载模型或使用 --model 参数指定模型")?
            }
        };

        // 验证模型文件
        self.model_manager.validate_model(&model_path)?;

        println!("正在解码音频文件: {:?}", file_path);

        // 解码音频
        let pcm_data = self.decoder.decode_to_pcm(&file_path)
            .map_err(|e| format!("音频解码失败: {}", e))?;

        if pcm_data.is_empty() {
            return Err("解码后的音频数据为空".to_string());
        }

        // 应用时间过滤
        let filtered_pcm_data = self.apply_time_filter(&pcm_data, skip_seconds, end_time)?;
        println!("时间过滤后样本数: {} (跳过前 {} 秒)", filtered_pcm_data.len(), skip_seconds);

        if filtered_pcm_data.is_empty() {
            return Err("时间过滤后音频数据为空".to_string());
        }

        println!("正在加载 Whisper 模型...");

        // 创建转录器
        let mut transcriber = WhisperTranscriber::new(&model_path.to_string_lossy())
            .map_err(|e| format!("无法初始化转录器: {}", e))?;

        println!("开始转录（带时间戳）...");

        // 执行转录获取分段
        let language_ref = language.as_deref();
        let segments = transcriber.transcribe(&filtered_pcm_data, language_ref)
            .map_err(|e| format!("转录失败: {}", e))?;

        // 调整时间戳以反映原始音频时间（加上跳过的时间）
        let adjusted_segments: Vec<_> = segments.iter().map(|segment| {
            let mut adjusted_segment = segment.clone();
            adjusted_segment.start_ms += (skip_seconds * 1000) as u32;
            adjusted_segment.end_ms += (skip_seconds * 1000) as u32;
            adjusted_segment
        }).collect();

        // 生成 SRT 格式输出
        let mut srt_content = String::new();
        for (index, segment) in adjusted_segments.iter().enumerate() {
            srt_content.push_str(&segment.to_srt_format(index + 1));
            srt_content.push('\n');
        }

        // 输出结果
        match output {
            Some(output_path) => {
                std::fs::write(&output_path, srt_content)
                    .map_err(|e| format!("无法写入输出文件: {}", e))?;
                println!("带时间戳的转录结果已保存到: {:?}", output_path);
            }
            None => {
                println!("\n=== 转录结果（带时间戳）===");
                println!("{}", srt_content);
                println!("============================");
            }
        }

        Ok(())
    }

    fn apply_time_filter(&self, pcm_data: &[i16], skip_seconds: u64, end_time: Option<u64>) -> Result<Vec<i16>, String> {
        // Whisper 音频采样率是 16kHz，即 16000 样本/秒
        const SAMPLE_RATE: u64 = 16000;

        let skip_samples = skip_seconds * SAMPLE_RATE;
        let end_samples = end_time.map(|t| t * SAMPLE_RATE);
        let total_samples = pcm_data.len() as u64;

        // 如果跳过的样本数超过了总样本数
        if skip_samples >= total_samples {
            return Err("跳过时间超过了音频总长度".to_string());
        }

        let start_idx = skip_samples as usize;
        let end_idx = match end_samples {
            Some(end) => {
                if end <= skip_samples {
                    return Err("结束时间不能小于或等于跳过时间".to_string());
                }
                std::cmp::min(end as usize, total_samples as usize)
            }
            None => total_samples as usize,
        };

        if start_idx >= end_idx {
            return Err("时间过滤后没有有效的音频数据".to_string());
        }

        Ok(pcm_data[start_idx..end_idx].to_vec())
    }

    pub async fn transcribe_file_with_advanced_params(
        &mut self,
        file_path: PathBuf,
        language: Option<String>,
        model: Option<String>,
        output: Option<PathBuf>,
        skip_seconds: u64,
        end_time: Option<u64>,
        temperature: f32,
        beam_size: i32,
        no_speech_threshold: f32,
    ) -> Result<(), String> {
        // 验证输入文件
        if !file_path.exists() {
            return Err(format!("音频文件不存在: {:?}", file_path));
        }

        // 确定使用的模型
        let model_path = match model {
            Some(model_input) => {
                // 智能查找模型：支持模型名称或完整路径
                self.model_manager.find_model_by_name_or_path(&model_input)
                    .ok_or_else(|| {
                        format!("找不到模型 '{}'。可用选项: tiny, base, small, medium, large，或者指定完整路径", model_input)
                    })?
            }
            None => {
                // 使用默认模型 (base)
                self.model_manager.get_model_path("base")
                    .ok_or("未找到默认模型 (base)。请先下载模型或使用 --model 参数指定模型")?
            }
        };

        // 验证模型文件
        self.model_manager.validate_model(&model_path)?;

        println!("正在解码音频文件: {:?}", file_path);

        // 显示解码进度
        let decode_progress = ProgressBar::new_spinner();
        decode_progress.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
        );
        decode_progress.set_message("解码音频中...");

        // 解码音频
        let pcm_data = self.decoder.decode_to_pcm(&file_path)
            .map_err(|e| format!("音频解码失败: {}", e))?;

        decode_progress.finish_with_message("音频解码完成");
        println!("音频解码完成，样本数: {}", pcm_data.len());

        if pcm_data.is_empty() {
            return Err("解码后的音频数据为空".to_string());
        }

        // 应用时间过滤
        let filtered_pcm_data = self.apply_time_filter(&pcm_data, skip_seconds, end_time)?;
        println!("时间过滤后样本数: {} (跳过前 {} 秒)", filtered_pcm_data.len(), skip_seconds);

        if filtered_pcm_data.is_empty() {
            return Err("时间过滤后音频数据为空".to_string());
        }

        println!("正在加载 Whisper 模型...");

        // 创建转录器
        let mut transcriber = WhisperTranscriber::new(&model_path.to_string_lossy())
            .map_err(|e| format!("无法初始化转录器: {}", e))?;

        println!("开始转录 (温度: {}, Beam大小: {}, 静音阈值: {})...", temperature, beam_size, no_speech_threshold);

        // 显示转录进度
        let transcribe_progress = ProgressBar::new_spinner();
        transcribe_progress.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.blue} {msg}")
                .unwrap()
        );
        transcribe_progress.set_message("转录中，请稍候...");

        // 执行转录
        let language_ref = language.as_deref();
        let result = transcriber.transcribe_to_text_with_params(&filtered_pcm_data, language_ref, temperature, beam_size, no_speech_threshold)
            .map_err(|e| format!("转录失败: {}", e))?;

        transcribe_progress.finish_with_message("转录完成");

        // 输出结果
        match output {
            Some(output_path) => {
                std::fs::write(&output_path, &result)
                    .map_err(|e| format!("无法写入输出文件: {}", e))?;
                println!("转录结果已保存到: {:?}", output_path);
            }
            None => {
                println!("\n=== 转录结果 ===");
                println!("{}", result);
                println!("==================");
            }
        }

        Ok(())
    }

    pub async fn transcribe_with_timestamps_and_advanced_params(
        &mut self,
        file_path: PathBuf,
        language: Option<String>,
        model: Option<String>,
        output: Option<PathBuf>,
        skip_seconds: u64,
        end_time: Option<u64>,
        temperature: f32,
        beam_size: i32,
        no_speech_threshold: f32,
    ) -> Result<(), String> {
        // 验证输入文件
        if !file_path.exists() {
            return Err(format!("音频文件不存在: {:?}", file_path));
        }

        // 确定使用的模型
        let model_path = match model {
            Some(model_input) => {
                // 智能查找模型：支持模型名称或完整路径
                self.model_manager.find_model_by_name_or_path(&model_input)
                    .ok_or_else(|| {
                        format!("找不到模型 '{}'。可用选项: tiny, base, small, medium, large，或者指定完整路径", model_input)
                    })?
            }
            None => {
                // 使用默认模型 (base)
                self.model_manager.get_model_path("base")
                    .ok_or("未找到默认模型 (base)。请先下载模型或使用 --model 参数指定模型")?
            }
        };

        // 验证模型文件
        self.model_manager.validate_model(&model_path)?;

        println!("正在解码音频文件: {:?}", file_path);

        // 解码音频
        let pcm_data = self.decoder.decode_to_pcm(&file_path)
            .map_err(|e| format!("音频解码失败: {}", e))?;

        if pcm_data.is_empty() {
            return Err("解码后的音频数据为空".to_string());
        }

        // 应用时间过滤
        let filtered_pcm_data = self.apply_time_filter(&pcm_data, skip_seconds, end_time)?;
        println!("时间过滤后样本数: {} (跳过前 {} 秒)", filtered_pcm_data.len(), skip_seconds);

        if filtered_pcm_data.is_empty() {
            return Err("时间过滤后音频数据为空".to_string());
        }

        println!("正在加载 Whisper 模型...");

        // 创建转录器
        let mut transcriber = WhisperTranscriber::new(&model_path.to_string_lossy())
            .map_err(|e| format!("无法初始化转录器: {}", e))?;

        println!("开始转录（带时间戳） (温度: {}, Beam大小: {}, 静音阈值: {})...", temperature, beam_size, no_speech_threshold);

        // 执行转录获取分段
        let language_ref = language.as_deref();
        let segments = transcriber.transcribe_with_params(&filtered_pcm_data, language_ref, temperature, beam_size, no_speech_threshold)
            .map_err(|e| format!("转录失败: {}", e))?;

        // 调整时间戳以反映原始音频时间（加上跳过的时间）
        let adjusted_segments: Vec<_> = segments.iter().map(|segment| {
            let mut adjusted_segment = segment.clone();
            adjusted_segment.start_ms += (skip_seconds * 1000) as u32;
            adjusted_segment.end_ms += (skip_seconds * 1000) as u32;
            adjusted_segment
        }).collect();

        // 生成 SRT 格式输出
        let mut srt_content = String::new();
        for (index, segment) in adjusted_segments.iter().enumerate() {
            srt_content.push_str(&segment.to_srt_format(index + 1));
            srt_content.push('\n');
        }

        // 输出结果
        match output {
            Some(output_path) => {
                std::fs::write(&output_path, srt_content)
                    .map_err(|e| format!("无法写入输出文件: {}", e))?;
                println!("带时间戳的转录结果已保存到: {:?}", output_path);
            }
            None => {
                println!("\n=== 转录结果（带时间戳）===");
                println!("{}", srt_content);
                println!("============================");
            }
        }

        Ok(())
    }

    pub async fn download_model(&mut self, model_name: &str) -> Result<(), String> {
        let available_models = ModelManager::get_available_models();
        let model_names: Vec<String> = available_models.iter().map(|m| m.name.clone()).collect();

        if !model_names.contains(&model_name.to_string()) {
            return Err(format!(
                "未知的模型大小: {}。可用选项: {}",
                model_name,
                model_names.join(", ")
            ));
        }

        println!("准备下载模型: {}", model_name);

        let model_path = self.model_manager.download_model(model_name).await?;

        // 验证下载的模型
        if let Err(e) = crate::services::audio::validate_whisper_model(&model_path.to_string_lossy()) {
            return Err(format!("模型下载失败，文件无效: {}", e));
        }

        println!("模型下载并验证成功！");
        Ok(())
    }

    pub fn list_models(&self) -> Result<(), String> {
        let available_models = ModelManager::get_available_models();
        let downloaded_models = self.model_manager.list_downloaded_models();

        println!("可用的 Whisper 模型:");
        println!("模型名称    大小      状态");
        println!("-----------------------------");

        for model in available_models {
            let status = if downloaded_models.contains(&model.name) {
                "已下载"
            } else {
                "未下载"
            };
            println!("{:<10} {:<8}  {}", model.name, model.size, status);
        }

        if downloaded_models.is_empty() {
            println!("\n提示: 使用 'scx-void audio download-model <model_name>' 下载模型");
        }

        Ok(())
    }
}

impl Default for AudioService {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            eprintln!("无法初始化音频服务: {}", e);
            panic!("音频服务初始化失败");
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_service_creation() {
        let service = AudioService::new();
        assert!(service.is_ok());
    }
}