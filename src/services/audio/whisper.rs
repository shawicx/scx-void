use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct WhisperTranscriber {
    context: WhisperContext,
}

impl WhisperTranscriber {
    pub fn new(model_path: &str) -> Result<Self, String> {
        let params = WhisperContextParameters::default();
        let context = WhisperContext::new_with_params(model_path, params)
            .map_err(|e| format!("无法加载 Whisper 模型: {}", e))?;

        Ok(Self { context })
    }

    #[allow(dead_code)]
    pub fn transcribe(&mut self, pcm_data: &[i16], language: Option<&str>) -> Result<Vec<TranscriptionSegment>, String> {
        self.transcribe_with_params(pcm_data, language, 0.3, 5, 0.6)
    }

    pub fn transcribe_with_params(&mut self, pcm_data: &[i16], language: Option<&str>, temperature: f32, beam_size: i32, no_speech_threshold: f32) -> Result<Vec<TranscriptionSegment>, String> {
        if pcm_data.is_empty() {
            return Err("音频数据为空".to_string());
        }

        // 将 i16 数据转换为 f32
        let pcm_f32: Vec<f32> = pcm_data.iter().map(|&s| s as f32 / 32768.0).collect();

        // 创建完整的参数 - 使用 Beam Search 提高准确性
        let mut params = FullParams::new(SamplingStrategy::BeamSearch {
            beam_size: beam_size.max(1),
            patience: 1.0
        });

        // 设置语言
        if let Some(lang) = language {
            params.set_language(Some(lang));
        } else {
            params.set_language(None); // 自动检测
        }

        // 禁用翻译
        params.set_translate(false);

        // 添加温度控制，提高转录自然度
        params.set_temperature(temperature);

        // 静音检测阈值，过滤风声和静音段
        params.set_no_speech_thold(no_speech_threshold);

        // 音频对数概率阈值，提高检测准确性
        params.set_logprob_thold(-1.0);

        // 启用时间戳
        params.set_single_segment(false);
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        // 开始转录
        let mut state = self.context.create_state()
            .map_err(|e| format!("创建状态失败: {}", e))?;

        state.full(params, &pcm_f32)
            .map_err(|e| format!("转录失败: {}", e))?;

        // 获取结果
        let num_segments = state.full_n_segments()
            .map_err(|e| format!("获取分段数量失败: {}", e))?;
        let mut segments = Vec::new();

        for i in 0..num_segments {
            let start_timestamp = state.full_get_segment_t0(i)
                .map_err(|e| format!("获取开始时间戳失败: {}", e))?;
            let end_timestamp = state.full_get_segment_t1(i)
                .map_err(|e| format!("获取结束时间戳失败: {}", e))?;
            let text = state.full_get_segment_text(i)
                .map_err(|e| format!("获取文本失败: {}", e))?;

            segments.push(TranscriptionSegment {
                start_ms: (start_timestamp * 10) as u32, // Whisper 返回 100ms 单位
                end_ms: (end_timestamp * 10) as u32,
                text: text.trim().to_string(),
            });
        }

        Ok(segments)
    }

    #[allow(dead_code)]
    pub fn transcribe_to_text(&mut self, pcm_data: &[i16], language: Option<&str>) -> Result<String, String> {
        self.transcribe_to_text_with_params(pcm_data, language, 0.3, 5, 0.6)
    }

    pub fn transcribe_to_text_with_params(&mut self, pcm_data: &[i16], language: Option<&str>, temperature: f32, beam_size: i32, no_speech_threshold: f32) -> Result<String, String> {
        let segments = self.transcribe_with_params(pcm_data, language, temperature, beam_size, no_speech_threshold)?;
        let text: String = segments.iter().map(|s| s.text.as_str()).collect::<Vec<_>>().join(" ");
        Ok(text)
    }
}

#[derive(Debug, Clone)]
pub struct TranscriptionSegment {
    pub start_ms: u32,
    pub end_ms: u32,
    pub text: String,
}

impl TranscriptionSegment {
    #[allow(dead_code)]
    pub fn duration_ms(&self) -> u32 {
        self.end_ms - self.start_ms
    }

    pub fn to_srt_format(&self, index: usize) -> String {
        let start_time = self.format_timestamp(self.start_ms);
        let end_time = self.format_timestamp(self.end_ms);
        format!("{}\n{} --> {}\n{}", index, start_time, end_time, self.text)
    }

    fn format_timestamp(&self, ms: u32) -> String {
        let seconds = ms / 1000;
        let minutes = seconds / 60;
        let hours = minutes / 60;

        let ms = ms % 1000;
        let seconds = seconds % 60;
        let minutes = minutes % 60;

        format!("{:02}:{:02}:{:02},{:03}", hours, minutes, seconds, ms)
    }
}

pub fn validate_whisper_model(model_path: &str) -> Result<(), String> {
    if !std::path::Path::new(model_path).exists() {
        return Err(format!("模型文件不存在: {}", model_path));
    }

    // 尝试加载模型来验证其有效性
    let params = WhisperContextParameters::default();
    let _ = WhisperContext::new_with_params(model_path, params)
        .map_err(|e| format!("无效的 Whisper 模型文件: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcription_segment() {
        let segment = TranscriptionSegment {
            start_ms: 1500,
            end_ms: 3500,
            text: "Hello world".to_string(),
        };

        assert_eq!(segment.duration_ms(), 2000);
        assert!(segment.to_srt_format(1).contains("Hello world"));
    }

    #[test]
    fn test_timestamp_formatting() {
        let segment = TranscriptionSegment {
            start_ms: 3661000, // 1:01:01.000
            end_ms: 3662000,   // 1:01:02.000
            text: "Test".to_string(),
        };

        let srt_line = segment.to_srt_format(1);
        assert!(srt_line.contains("01:01:01,000"));
        assert!(srt_line.contains("01:01:02,000"));
    }

    #[test]
    fn test_validate_model_nonexistent() {
        let result = validate_whisper_model("/nonexistent/model.bin");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("模型文件不存在"));
    }
}