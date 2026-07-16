use crate::errors::ScxVoidError;
use crate::services::convert::heic;
use crate::services::convert::ImageFormat;
use std::path::Path;

/// 目标格式标识（字符串形式，便于扩展，如 "png"、"jpeg"）
pub type TargetFormat<'a> = &'a str;

/// HEIC magic bytes 校验：ISO BMFF ftyp box，主品牌为 heic 家族或 mif1/msf1。
/// 输入应为文件头前 32 字节。
pub fn is_heic_magic(bytes: &[u8]) -> bool {
    // 至少需要读到 ftyp box 头：偏移 4..12
    if bytes.len() < 12 {
        return false;
    }
    let ftyp = &bytes[4..8];
    let brand = &bytes[8..12];
    if ftyp != b"ftyp" {
        return false;
    }
    matches!(
        brand,
        b"heic" | b"heix" | b"hevc" | b"heim" | b"heis" | b"mif1" | b"msf1"
    )
}

/// 根据扩展名推断源格式。不区分大小写。
pub fn format_from_extension(path: &Path) -> Option<ImageFormat> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    match ext.as_str() {
        "heic" | "heif" => Some(ImageFormat::Heic),
        _ => None,
    }
}

/// 校验文件头与扩展名是否一致，返回确认的源格式。
/// - 扩展名无法识别 → UnsupportedImageFormat
/// - 扩展名识别为 HEIC 但 magic bytes 不符 → UnsupportedImageFormat（扩展名与内容不符）
pub fn detect_format(path: &Path, head_bytes: &[u8]) -> Result<ImageFormat, ScxVoidError> {
    let by_ext = format_from_extension(path).ok_or_else(|| {
        ScxVoidError::UnsupportedImageFormat(format!(
            "无法识别的扩展名: {:?}",
            path.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("(无)")
        ))
    })?;

    match by_ext {
        ImageFormat::Heic => {
            if is_heic_magic(head_bytes) {
                Ok(ImageFormat::Heic)
            } else {
                Err(ScxVoidError::UnsupportedImageFormat(
                    "扩展名指示 HEIC，但文件头不符".to_string(),
                ))
            }
        }
    }
}

/// 查询某源格式可转换的目标格式列表。
/// 首期只支持 HEIC → PNG。
pub fn target_formats(source: ImageFormat) -> Vec<TargetFormat<'static>> {
    match source {
        ImageFormat::Heic => vec!["png"],
    }
}

/// 校验目标格式是否在该源格式的可转列表中。
pub fn is_supported_target(source: ImageFormat, target: TargetFormat) -> bool {
    target_formats(source).contains(&target)
}

/// 分派执行转换。调用方需先确保 (source, target) 已通过 is_supported_target 校验。
pub fn dispatch_convert(
    source: ImageFormat,
    input: &std::path::Path,
    output: &std::path::Path,
    target: TargetFormat,
) -> Result<(), ScxVoidError> {
    match source {
        ImageFormat::Heic => heic::convert_heic(input, output, target),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn heic_head() -> Vec<u8> {
        // 真实 HEIC 文件头样本：偏移 4..12 = "ftypheic"
        let mut v = vec![0u8; 32];
        v[4..8].copy_from_slice(b"ftyp");
        v[8..12].copy_from_slice(b"heic");
        v
    }

    #[test]
    fn test_is_heic_magic_valid_brands() {
        for brand in [b"heic", b"heix", b"hevc", b"mif1", b"msf1"] {
            let mut v = vec![0u8; 32];
            v[4..8].copy_from_slice(b"ftyp");
            v[8..12].copy_from_slice(brand);
            assert!(is_heic_magic(&v), "brand {:?} 应识别为 heic", brand);
        }
    }

    #[test]
    fn test_is_heic_magic_invalid() {
        assert!(!is_heic_magic(&[]));
        assert!(!is_heic_magic(&[0u8; 12]));
        let mut v = vec![0u8; 32];
        v[4..8].copy_from_slice(b"ftyp");
        v[8..12].copy_from_slice(b"jpeg");
        assert!(!is_heic_magic(&v));
    }

    #[test]
    fn test_format_from_extension() {
        assert_eq!(format_from_extension(Path::new("a.heic")), Some(ImageFormat::Heic));
        assert_eq!(format_from_extension(Path::new("A.HEIF")), Some(ImageFormat::Heic));
        assert_eq!(format_from_extension(Path::new("a.png")), None);
        assert_eq!(format_from_extension(Path::new("noext")), None);
    }

    #[test]
    fn test_detect_format_success() {
        let head = heic_head();
        assert_eq!(detect_format(Path::new("photo.heic"), &head).unwrap(), ImageFormat::Heic);
    }

    #[test]
    fn test_detect_format_unknown_extension() {
        let err = detect_format(Path::new("a.png"), &[0u8; 32]).unwrap_err();
        assert!(matches!(err, ScxVoidError::UnsupportedImageFormat(_)));
    }

    #[test]
    fn test_detect_format_magic_mismatch() {
        let err = detect_format(Path::new("a.heic"), &[0u8; 32]).unwrap_err();
        assert!(matches!(err, ScxVoidError::UnsupportedImageFormat(_)));
    }

    #[test]
    fn test_target_formats_heic() {
        assert_eq!(target_formats(ImageFormat::Heic), vec!["png"]);
    }

    #[test]
    fn test_is_supported_target() {
        assert!(is_supported_target(ImageFormat::Heic, "png"));
        assert!(!is_supported_target(ImageFormat::Heic, "jpeg"));
    }

    #[test]
    fn test_default_output_path() {
        use crate::services::convert::default_output_path;
        let p = default_output_path(Path::new("/photos/trip.heic"), "png");
        assert_eq!(p, Path::new("/photos/trip.png"));
    }
}
