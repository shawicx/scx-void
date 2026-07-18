use crate::services::compress::format;
use crate::services::compress::webp;
use crate::services::compress::{
    default_compress_output_path, format_size, savings_percent, QualityPreset,
};
use dialoguer::{Input, Select};
use std::path::PathBuf;

/// 执行图片压缩的交互流程。
///
/// 由顶层 `compress` 命令直接调用（扁平命令，非子命令组）。
pub async fn run_compress(
    file: Option<String>,
    quality: Option<u8>,
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

    // 2. 读取文件头做格式检测
    let mut head = [0u8; 16];
    {
        use std::io::Read;
        let mut f = std::fs::File::open(&input)?;
        let _ = f.read(&mut head);
    }
    let _fmt = format::detect_format(&input, &head)?;
    // fmt 仅用于校验，所有支持格式都转 WebP，无需分支

    // 3. 质量
    let q = match quality {
        Some(q) if (1..=100).contains(&q) => q,
        Some(q) => return Err(format!("质量参数必须在 1-100 之间，当前: {}", q).into()),
        None => {
            let labels = QualityPreset::all_labels();
            let idx = Select::new()
                .with_prompt("选择压缩质量")
                .items(&labels)
                .default(1) // medium 最常用
                .interact()?;
            QualityPreset::from_index(idx)
                .ok_or_else(|| "无效的质量选择".to_string())?
                .value()
        }
    };

    // 4. 输出路径
    let out_path = match output {
        Some(o) => PathBuf::from(o),
        None => default_compress_output_path(&input),
    };

    // 5. 冲突检查
    if out_path.exists() && !overwrite {
        return Err(format!(
            "输出文件已存在: {}（使用 --overwrite 覆盖）",
            out_path.display()
        )
        .into());
    }

    // 6. 记录原始体积并执行压缩
    let original_size = std::fs::metadata(&input)?.len();
    webp::compress_to_webp(&input, &out_path, q)?;
    let compressed_size = std::fs::metadata(&out_path)?.len();

    // 7. 输出结果 + 体积对比
    println!("✓ 已生成: {}", out_path.display());
    println!(
        "  原始: {} → 压缩后: {} (节省 {}%)",
        format_size(original_size),
        format_size(compressed_size),
        savings_percent(original_size, compressed_size)
    );
    Ok(())
}
