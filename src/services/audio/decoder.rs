use std::path::Path;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

pub struct AudioDecoder {
    sample_rate: u32,
    channels: u32,
}

impl AudioDecoder {
    pub fn new() -> Self {
        Self {
            sample_rate: 16000, // Whisper 推荐的采样率
            channels: 1,        // 单声道
        }
    }

    pub fn decode_to_pcm(&self, file_path: &Path) -> Result<Vec<i16>, String> {
        // 打开音频文件
        let file = std::fs::File::open(file_path)
            .map_err(|e| format!("无法打开文件: {}", e))?;

        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        // 创建格式提示
        let mut hint = Hint::new();
        if let Some(extension) = file_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                hint.with_extension(ext_str);
            }
        }

        // 探测格式
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .map_err(|e| format!("不支持的音频格式: {}", e))?;

        let mut format = probed.format;

        // 找到第一个音频轨道
        let (track_id, codec_params) = {
            let tracks = format.tracks();
            let track = tracks.iter()
                .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
                .ok_or("未找到音频轨道")?;
            (track.id, track.codec_params.clone())
        };

        // 创建解码器
        let decoder = symphonia::default::get_codecs()
            .make(&codec_params, &Default::default())
            .map_err(|e| format!("无法创建解码器: {}", e))?;

        let mut decoder = decoder;

        let mut pcm_data = Vec::new();

        // 解码音频数据
        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(SymphoniaError::ResetRequired) => break,
                Err(SymphoniaError::IoError(ref err))
                    if err.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(err) => return Err(format!("解码错误: {}", err)),
            };

            if packet.track_id() != track_id {
                continue;
            }

            match decoder.decode(&packet) {
                Ok(decoded) => {
                    // 处理解码后的音频数据
                    match decoded {
                        symphonia::core::audio::AudioBufferRef::F32(buf) => {
                            for plane in buf.planes().planes() {
                                let samples: &[f32] = plane;
                                for &sample in samples {
                                    // 转换为 16-bit PCM
                                    let sample_i16 = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                                    pcm_data.push(sample_i16);
                                }
                            }
                        }
                        symphonia::core::audio::AudioBufferRef::S16(buf) => {
                            for plane in buf.planes().planes() {
                                let samples: &[i16] = plane;
                                for &sample in samples {
                                    pcm_data.push(sample);
                                }
                            }
                        }
                        // 暂时只处理 F32 和 S16 格式
                        _ => return Err("不支持的音频格式".to_string()),
                    }
                }
                Err(SymphoniaError::IoError(_)) => continue,
                Err(SymphoniaError::DecodeError(_)) => continue,
                Err(err) => return Err(format!("解码失败: {}", err)),
            }
        }

        if pcm_data.is_empty() {
            return Err("未解码到任何音频数据".to_string());
        }

        // 如果不是单声道，需要下混
        let original_channels = codec_params.channels.map(|c| c.count()).unwrap_or(1);
        if original_channels > 1 {
            pcm_data = self.downmix_to_mono(pcm_data, original_channels as usize);
        }

        // 重采样到 16kHz（如果需要）
        let original_sample_rate = codec_params.sample_rate.unwrap_or(44100);
        if original_sample_rate != self.sample_rate {
            pcm_data = self.resample(pcm_data, original_sample_rate, self.sample_rate);
        }

        Ok(pcm_data)
    }

    fn downmix_to_mono(&self, pcm_data: Vec<i16>, channels: usize) -> Vec<i16> {
        if channels == 1 {
            return pcm_data;
        }

        let samples_per_channel = pcm_data.len() / channels;
        let mut mono_data = Vec::with_capacity(samples_per_channel);

        for i in 0..samples_per_channel {
            let mut sum = 0i32;
            for ch in 0..channels {
                sum += pcm_data[i * channels + ch] as i32;
            }
            mono_data.push((sum / channels as i32) as i16);
        }

        mono_data
    }

    fn resample(&self, pcm_data: Vec<i16>, from_rate: u32, to_rate: u32) -> Vec<i16> {
        if from_rate == to_rate {
            return pcm_data;
        }

        let ratio = to_rate as f64 / from_rate as f64;
        let new_length = (pcm_data.len() as f64 * ratio) as usize;
        let mut resampled = Vec::with_capacity(new_length);

        for i in 0..new_length {
            let src_index = (i as f64 / ratio) as usize;
            if src_index < pcm_data.len() {
                resampled.push(pcm_data[src_index]);
            }
        }

        resampled
    }
}

impl Default for AudioDecoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_decoder_creation() {
        let decoder = AudioDecoder::new();
        assert_eq!(decoder.sample_rate, 16000);
        assert_eq!(decoder.channels, 1);
    }

    #[test]
    fn test_downmix_to_mono() {
        let decoder = AudioDecoder::new();
        // 模拟立体声数据: [L1, R1, L2, R2, ...]
        let stereo_data = vec![1000, 2000, 3000, 4000];
        let mono = decoder.downmix_to_mono(stereo_data, 2);
        assert_eq!(mono, vec![1500, 3500]); // (1000+2000)/2, (3000+4000)/2
    }
}