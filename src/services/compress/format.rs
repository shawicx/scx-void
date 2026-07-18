use crate::errors::ScxVoidError;
use std::path::Path;

/// 可压缩的图片格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressFormat {
    Jpeg,
    Png,
    Webp,
}

/// 根据文件头判定格式（magic bytes 权威，扩展名仅用于错误提示）。
/// `head` 应为文件头前 16 字节。
pub fn detect_format(path: &Path, head: &[u8]) -> Result<CompressFormat, ScxVoidError> {
    if is_jpeg(head) {
        return Ok(CompressFormat::Jpeg);
    }
    if is_png(head) {
        return Ok(CompressFormat::Png);
    }
    if is_webp(head) {
        return Ok(CompressFormat::Webp);
    }
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("(无扩展名)");
    Err(ScxVoidError::UnsupportedCompressFormat(format!(
        "无法识别的图片格式（扩展名: {}）。仅支持 JPEG / PNG / WebP",
        ext
    )))
}

/// JPEG: FF D8 FF
fn is_jpeg(head: &[u8]) -> bool {
    head.len() >= 3 && head[0..3] == [0xFF, 0xD8, 0xFF]
}

/// PNG: 89 50 4E 47 0D 0A 1A 0A
fn is_png(head: &[u8]) -> bool {
    head.len() >= 8 && head[0..8] == [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
}

/// WebP: "RIFF" .... "WEBP"
fn is_webp(head: &[u8]) -> bool {
    head.len() >= 12 && &head[0..4] == b"RIFF" && &head[8..12] == b"WEBP"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_jpeg() {
        let head = [0xFF, 0xD8, 0xFF, 0xE0, 0x00];
        assert_eq!(
            detect_format(Path::new("a.jpg"), &head).unwrap(),
            CompressFormat::Jpeg
        );
    }

    #[test]
    fn test_detect_png() {
        let mut head = [0u8; 16];
        head[0..8].copy_from_slice(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
        assert_eq!(
            detect_format(Path::new("a.png"), &head).unwrap(),
            CompressFormat::Png
        );
    }

    #[test]
    fn test_detect_webp() {
        let mut head = [0u8; 16];
        head[0..4].copy_from_slice(b"RIFF");
        head[8..12].copy_from_slice(b"WEBP");
        assert_eq!(
            detect_format(Path::new("a.webp"), &head).unwrap(),
            CompressFormat::Webp
        );
    }

    #[test]
    fn test_detect_unsupported() {
        // GIF 文件头
        let head = b"GIF89a";
        let err = detect_format(Path::new("a.gif"), head).unwrap_err();
        assert!(matches!(err, ScxVoidError::UnsupportedCompressFormat(_)));
    }

    #[test]
    fn test_detect_empty_head() {
        let err = detect_format(Path::new("a.jpg"), &[]).unwrap_err();
        assert!(matches!(err, ScxVoidError::UnsupportedCompressFormat(_)));
    }
}
