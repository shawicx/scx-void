use std::path::{Path, PathBuf};
use std::fs;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::io::AsyncWriteExt;

pub struct ModelManager {
    models_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub size: String,
    pub download_url: String,
    pub file_size_mb: u64,
}

impl ModelManager {
    pub fn new() -> Result<Self, String> {
        let home_dir = dirs::home_dir().ok_or("无法获取用户主目录")?;
        let models_dir = home_dir.join(".scx-void").join("models");

        // 确保模型目录存在
        fs::create_dir_all(&models_dir)
            .map_err(|e| format!("无法创建模型目录: {}", e))?;

        Ok(Self { models_dir })
    }

    pub fn get_available_models() -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                name: "tiny".to_string(),
                size: "39 MB".to_string(),
                download_url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin".to_string(),
                file_size_mb: 39,
            },
            ModelInfo {
                name: "base".to_string(),
                size: "74 MB".to_string(),
                download_url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin".to_string(),
                file_size_mb: 74,
            },
            ModelInfo {
                name: "small".to_string(),
                size: "244 MB".to_string(),
                download_url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin".to_string(),
                file_size_mb: 244,
            },
            ModelInfo {
                name: "medium".to_string(),
                size: "769 MB".to_string(),
                download_url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin".to_string(),
                file_size_mb: 769,
            },
            ModelInfo {
                name: "large".to_string(),
                size: "1550 MB".to_string(),
                download_url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large.bin".to_string(),
                file_size_mb: 1550,
            },
        ]
    }

    pub async fn download_model(&self, model_name: &str) -> Result<PathBuf, String> {
        let models = Self::get_available_models();
        let model_info = models.iter()
            .find(|m| m.name == model_name)
            .ok_or_else(|| format!("未知的模型大小: {}。可用选项: tiny, base, small, medium, large",
                model_name))?;

        let filename = format!("ggml-{}.bin", model_name);
        let model_path = self.models_dir.join(&filename);

        // 检查模型是否已存在
        if model_path.exists() {
            println!("模型 {} 已存在于: {:?}", model_name, model_path);
            return Ok(model_path);
        }

        println!("开始下载模型 {} ({})...", model_name, model_info.size);

        let progress_bar = ProgressBar::new(model_info.file_size_mb * 1024 * 1024);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .progress_chars("#>-")
        );

        // 发起 HTTP 请求
        let client = reqwest::Client::new();
        let response = client.get(&model_info.download_url)
            .send()
            .await
            .map_err(|e| format!("下载失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("下载失败: HTTP {}", response.status()));
        }

        let total_size = response.content_length()
            .ok_or("无法获取文件大小")?;

        progress_bar.set_length(total_size);

        // 创建临时文件
        let temp_path = model_path.with_extension("tmp");
        let mut file = tokio::fs::File::create(&temp_path).await
            .map_err(|e| format!("无法创建临时文件: {}", e))?;

        let mut downloaded = 0u64;
        let mut stream = response.bytes_stream();

        use futures_util::StreamExt;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| format!("下载中断: {}", e))?;
            file.write_all(&chunk).await
                .map_err(|e| format!("写入文件失败: {}", e))?;

            downloaded += chunk.len() as u64;
            progress_bar.set_position(downloaded);
        }

        file.flush().await
            .map_err(|e| format!("刷新文件失败: {}", e))?;
        drop(file);

        // 重命名临时文件为目标文件
        fs::rename(&temp_path, &model_path)
            .map_err(|e| format!("保存模型文件失败: {}", e))?;

        progress_bar.finish_with_message("下载完成");
        println!("模型已保存到: {:?}", model_path);

        Ok(model_path)
    }

    pub fn get_model_path(&self, model_name: &str) -> Option<PathBuf> {
        // 首先尝试标准命名: ggml-{model_name}.bin
        let filename = format!("ggml-{}.bin", model_name);
        let model_path = self.models_dir.join(&filename);
        if model_path.exists() {
            return Some(model_path);
        }

        // 如果标准命名不存在，列出所有模型文件
        if let Ok(entries) = fs::read_dir(&self.models_dir) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    // 检查文件名是否包含模型名称（不区分大小写）
                    if file_name.to_lowercase().contains(&model_name.to_lowercase()) && file_name.ends_with(".bin") {
                        println!("找到匹配的模型文件: {}", file_name);
                        return Some(entry.path());
                    }
                }
            }
        }

        None
    }

    pub fn find_model_by_name_or_path(&self, model_input: &str) -> Option<PathBuf> {
        // 如果输入是完整路径，直接检查
        let input_path = PathBuf::from(model_input);
        if input_path.exists() && input_path.is_file() {
            return Some(input_path);
        }

        // 如果输入不是完整路径，在模型目录中查找
        self.get_model_path(model_input)
    }

    pub fn list_downloaded_models(&self) -> Vec<String> {
        let mut models = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.models_dir) {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.starts_with("ggml-") && filename.ends_with(".bin") {
                        // 提取模型名称 (ggml-tiny.bin -> tiny)
                        if let Some(model_name) = filename.strip_prefix("ggml-").and_then(|s| s.strip_suffix(".bin")) {
                            models.push(model_name.to_string());
                        }
                    }
                }
            }
        }

        models.sort();
        models
    }

    pub fn validate_model(&self, model_path: &Path) -> Result<(), String> {
        if !model_path.exists() {
            return Err(format!("模型文件不存在: {:?}", model_path));
        }

        let metadata = model_path.metadata()
            .map_err(|e| format!("无法读取模型文件信息: {}", e))?;

        if metadata.len() == 0 {
            return Err("模型文件为空".to_string());
        }

        // 可以添加更多验证逻辑，比如检查文件头等
        Ok(())
    }
}

impl Default for ModelManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            eprintln!("警告: 无法创建模型管理器: {}", e);
            panic!("模型管理器初始化失败");
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_models() {
        let models = ModelManager::get_available_models();
        assert!(!models.is_empty());

        // 检查是否包含基础模型
        let model_names: Vec<String> = models.iter().map(|m| m.name.clone()).collect();
        assert!(model_names.contains(&"tiny".to_string()));
        assert!(model_names.contains(&"base".to_string()));
        assert!(model_names.contains(&"small".to_string()));
    }

    #[test]
    fn test_model_info() {
        let tiny = ModelManager::get_available_models()
            .into_iter()
            .find(|m| m.name == "tiny")
            .unwrap();

        assert_eq!(tiny.name, "tiny");
        assert!(tiny.download_url.contains("ggml-tiny.bin"));
        assert!(tiny.file_size_mb > 30);
    }
}