pub mod format;
pub mod webp;

/// 压缩质量预设
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityPreset {
    High,
    Medium,
    Low,
}

impl QualityPreset {
    /// 对应的 cwebp -q 数值
    pub fn value(&self) -> u8 {
        match self {
            QualityPreset::High => 85,
            QualityPreset::Medium => 75,
            QualityPreset::Low => 60,
        }
    }

    /// 交互选择时显示的标签（带数值便于决策）
    pub fn all_labels() -> Vec<&'static str> {
        vec!["high (85)", "medium (75)", "low (60)"]
    }

    /// 从选择索引构造预设
    pub fn from_index(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(QualityPreset::High),
            1 => Some(QualityPreset::Medium),
            2 => Some(QualityPreset::Low),
            _ => None,
        }
    }
}

/// 计算默认压缩输出路径：与输入同目录、同名、后缀换 .webp。
pub fn default_compress_output_path(input: &std::path::Path) -> std::path::PathBuf {
    let mut out = input.to_path_buf();
    out.set_extension("webp");
    out
}

/// 格式化字节数为人类可读（KB 或 MB）。
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * 1024;
    if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// 计算节省百分比（向下取整）。original 为 0 时返回 0。
pub fn savings_percent(original: u64, compressed: u64) -> u64 {
    if original == 0 {
        return 0;
    }
    let saved = original.saturating_sub(compressed);
    saved * 100 / original
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_preset_values() {
        assert_eq!(QualityPreset::High.value(), 85);
        assert_eq!(QualityPreset::Medium.value(), 75);
        assert_eq!(QualityPreset::Low.value(), 60);
    }

    #[test]
    fn test_quality_preset_from_index() {
        assert_eq!(QualityPreset::from_index(0), Some(QualityPreset::High));
        assert_eq!(QualityPreset::from_index(2), Some(QualityPreset::Low));
        assert_eq!(QualityPreset::from_index(3), None);
    }

    #[test]
    fn test_default_compress_output_path() {
        assert_eq!(
            default_compress_output_path(std::path::Path::new("/a/b.jpg")),
            std::path::Path::new("/a/b.webp")
        );
        assert_eq!(
            default_compress_output_path(std::path::Path::new("/a/b.webp")),
            std::path::Path::new("/a/b.webp")
        );
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1_200_000), "1.14 MB");
    }

    #[test]
    fn test_savings_percent() {
        assert_eq!(savings_percent(1000, 200), 80);
        assert_eq!(savings_percent(1000, 1000), 0);
        assert_eq!(savings_percent(0, 0), 0);
        assert_eq!(savings_percent(7858, 252), 96);
    }
}
