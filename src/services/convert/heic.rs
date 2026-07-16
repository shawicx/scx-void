use crate::errors::ScxVoidError;
use indicatif::ProgressBar;
use std::path::Path;

/// 将 HEIC 文件转换为指定目标格式（首期仅 png）。
///
/// 平台工具映射：
/// - macOS: sips（系统自带）
/// - Linux/Windows: ImageMagick magick（用户预装）
pub fn convert_heic(input: &Path, output: &Path, target: &str) -> Result<(), ScxVoidError> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_message(format!("转换 HEIC → {} ...", target));
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let result = run_conversion(input, output, target);

    spinner.finish_with_message(format!("转换完成: {}", output.display()));
    result
}

#[cfg(target_os = "macos")]
fn run_conversion(input: &Path, output: &Path, target: &str) -> Result<(), ScxVoidError> {
    const TOOL: &str = "sips";
    ensure_tool(TOOL, "sips 是 macOS 系统自带工具，若缺失请检查 macOS 系统完整性")?;

    let output_str = output.to_string_lossy().to_string();
    let input_str = input.to_string_lossy().to_string();
    let handle = duct::cmd!(TOOL, "-s", "format", target, &input_str, "--out", &output_str)
        .stderr_capture()
        .run();

    finish_or_error(handle, input, output, target, TOOL)
}

#[cfg(not(target_os = "macos"))]
fn run_conversion(input: &Path, output: &Path, target: &str) -> Result<(), ScxVoidError> {
    const TOOL: &str = "magick";
    let hint = match std::env::consts::OS {
        "linux" => "请安装 ImageMagick，例如: sudo apt install imagemagick".to_string(),
        "windows" => {
            "请安装 ImageMagick: winget install ImageMagick.ImageMagick 或访问 https://imagemagick.org"
                .to_string()
        }
        other => format!("请安装 ImageMagick（当前平台 {}）", other),
    };
    ensure_tool(TOOL, &hint)?;

    let handle = duct::cmd!(
        TOOL,
        input.to_string_lossy().as_ref(),
        output.to_string_lossy().as_ref()
    )
    .stderr_capture()
    .run();

    // magick 通过输出扩展名决定目标格式，target 参数仅用于错误信息
    finish_or_error(handle, input, output, target, TOOL)
}

/// 检测工具是否存在（which/where）。不存在返回 ConverterNotFound。
fn ensure_tool(tool: &str, hint: &str) -> Result<(), ScxVoidError> {
    let finder = if cfg!(target_os = "windows") {
        "where"
    } else {
        "which"
    };
    let check = duct::cmd!(finder, tool).stdout_capture().run();
    if check.is_err() {
        return Err(ScxVoidError::ConverterNotFound {
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
    target: &str,
    tool: &str,
) -> Result<(), ScxVoidError> {
    let output_obj = handle.map_err(|e| ScxVoidError::ImageConversionFailed {
        source: input.display().to_string(),
        target: target.to_string(),
        reason: format!("{} 执行失败: {}", tool, e),
    })?;

    if !output_obj.status.success() {
        let stderr = String::from_utf8_lossy(&output_obj.stderr);
        return Err(ScxVoidError::ImageConversionFailed {
            source: input.display().to_string(),
            target: target.to_string(),
            reason: format!(
                "{} 退出码 {:?}: {}",
                tool,
                output_obj.status.code(),
                stderr.trim()
            ),
        });
    }

    if !output.exists() {
        return Err(ScxVoidError::ImageConversionFailed {
            source: input.display().to_string(),
            target: target.to_string(),
            reason: format!(
                "{} 执行成功但输出文件未生成: {}",
                tool,
                output.display()
            ),
        });
    }
    Ok(())
}
