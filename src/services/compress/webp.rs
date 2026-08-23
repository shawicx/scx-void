use crate::errors::ScxVoidError;
use indicatif::ProgressBar;
use std::path::Path;

/// 将图片压缩为 WebP。
///
/// 调用系统工具 cwebp（Google libwebp）。缺失时返回 CompressorNotFound。
pub fn compress_to_webp(
    input: &Path,
    output: &Path,
    quality: u8,
) -> Result<(), ScxVoidError> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_message(format!("压缩为 WebP (q={}) ...", quality));
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let result = run_cwebp(input, output, quality);

    spinner.finish_with_message(format!("压缩完成: {}", output.display()));
    result
}

fn run_cwebp(input: &Path, output: &Path, quality: u8) -> Result<(), ScxVoidError> {
    const TOOL: &str = "cwebp";
    ensure_tool(TOOL, &platform_hint())?;

    let handle = duct::cmd!(
        TOOL,
        "-q",
        quality.to_string(),
        input.to_string_lossy().as_ref(),
        "-o",
        output.to_string_lossy().as_ref()
    )
    .stderr_capture()
    .run();

    finish_or_error(handle, input, output, TOOL)
}

/// 平台相关的安装提示
fn platform_hint() -> String {
    match std::env::consts::OS {
        "macos" => "请安装 libwebp: brew install webp".to_string(),
        "linux" => "请安装 libwebp: sudo apt install webp".to_string(),
        "windows" => {
            "请安装 libwebp: winget install Google.WebP 或访问 https://developers.google.com/speed/webp"
                .to_string()
        }
        other => format!("请安装 libwebp（当前平台 {}）", other),
    }
}

/// 检测工具是否存在。不存在返回 CompressorNotFound。
fn ensure_tool(tool: &str, hint: &str) -> Result<(), ScxVoidError> {
    let finder = if cfg!(target_os = "windows") {
        "where"
    } else {
        "which"
    };
    let check = duct::cmd!(finder, tool).stdout_capture().run();
    if check.is_err() {
        return Err(ScxVoidError::CompressorNotFound {
            tool: tool.to_string(),
            hint: hint.to_string(),
        });
    }
    Ok(())
}

/// 统一处理命令结果：成功且输出文件确实生成才算成功。
fn finish_or_error(
    handle: std::io::Result<std::process::Output>,
    input: &Path,
    output: &Path,
    tool: &str,
) -> Result<(), ScxVoidError> {
    let output_obj = handle.map_err(|e| ScxVoidError::CompressionFailed {
        source: input.display().to_string(),
        reason: format!("{} 执行失败: {}", tool, e),
    })?;

    if !output_obj.status.success() {
        let stderr = String::from_utf8_lossy(&output_obj.stderr);
        return Err(ScxVoidError::CompressionFailed {
            source: input.display().to_string(),
            reason: format!(
                "{} 退出码 {:?}: {}",
                tool,
                output_obj.status.code(),
                stderr.trim()
            ),
        });
    }

    if !output.exists() {
        return Err(ScxVoidError::CompressionFailed {
            source: input.display().to_string(),
            reason: format!("{} 执行成功但输出文件未生成: {}", tool, output.display()),
        });
    }
    Ok(())
}
