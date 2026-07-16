use crate::services::convert::registry;
use crate::services::convert::default_output_path;
use dialoguer::{Input, Select};
use std::path::PathBuf;

/// 执行文件格式转换的交互流程。
///
/// 由顶层 `convert` 命令直接调用（扁平命令，非子命令组）。
/// 参数均来自 clap 解析；缺失的 file/format 通过 dialoguer 交互补全。
pub async fn run_convert(
    file: Option<String>,
    format: Option<String>,
    output: Option<String>,
    overwrite: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. 获取输入路径
    let input_str = match file {
        Some(f) => f,
        None => Input::<String>::new()
            .with_prompt("输入文件路径")
            .interact()?
            .trim()
            .to_string(),
    };
    let input = PathBuf::from(&input_str);

    if !input.exists() {
        return Err(format!("文件不存在: {}", input.display()).into());
    }

    // 2. 读取文件头做 magic bytes 校验
    let mut head = [0u8; 32];
    {
        use std::io::Read;
        let mut f = std::fs::File::open(&input)?;
        let _ = f.read(&mut head); // 文件可能不足 32 字节，忽略已读字节数
    }

    // 3. 检测源格式
    let source = registry::detect_format(&input, &head)?;
    println!("检测到格式: {}", source.as_str().to_uppercase());

    // 4. 目标格式
    let targets = registry::target_formats(source);
    if targets.is_empty() {
        return Err(format!("格式 {} 暂无可转换的目标格式", source.as_str()).into());
    }
    let target: String = match format {
        Some(f) => {
            let f = f.trim().to_ascii_lowercase();
            if !registry::is_supported_target(source, f.as_str()) {
                return Err(format!(
                    "不支持的目标格式 '{}'，可选: {}",
                    f,
                    targets.join(", ")
                )
                .into());
            }
            f
        }
        None => {
            let idx = Select::new()
                .with_prompt("选择目标格式")
                .items(&targets)
                .default(0)
                .interact()?;
            targets[idx].to_string()
        }
    };

    // 5. 计算输出路径
    let out_path = match output {
        Some(o) => PathBuf::from(o),
        None => default_output_path(&input, &target),
    };

    // 6. 冲突检查
    if out_path.exists() && !overwrite {
        return Err(format!(
            "输出文件已存在: {}（使用 --overwrite 覆盖）",
            out_path.display()
        )
        .into());
    }

    // 7. 执行转换
    registry::dispatch_convert(source, &input, &out_path, &target)?;
    println!("✓ 已生成: {}", out_path.display());
    Ok(())
}
