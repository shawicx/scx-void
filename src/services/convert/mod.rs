pub mod heic;
pub mod registry;

/// 检测到的源图像格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Heic,
}

impl ImageFormat {
    /// 用于扩展名匹配与显示的字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            ImageFormat::Heic => "heic",
        }
    }
}

/// 转换结果输出路径的计算
///
/// 默认：与输入同目录，文件主名不变，扩展名替换为目标格式。
pub fn default_output_path(input: &std::path::Path, target_ext: &str) -> std::path::PathBuf {
    let mut out = input.to_path_buf();
    out.set_extension(target_ext);
    out
}
