use crate::services::compress::webp;
use crate::services::compress::{
    default_compress_output_path, format_size, savings_percent, QualityPreset,
};
use crate::tui::actions::{
    ActionGroup, ActionMeta, FieldSpec, FieldValues, TaskCtx, TaskOutcome, TuiAction, TuiTask,
};
use std::path::PathBuf;

pub struct CompressAction;

impl TuiAction for CompressAction {
    fn meta(&self) -> ActionMeta {
        ActionMeta {
            id: "compress",
            title: "图片压缩 (WebP)",
            description: "将 JPEG/PNG/WebP 图片压缩为 WebP，展示体积对比".into(),
            group: ActionGroup::File,
        }
    }

    fn fields(&self) -> Vec<FieldSpec> {
        vec![
            FieldSpec::Text {
                key: "file",
                label: "输入图片路径".into(),
                placeholder: Some("photo.jpg".into()),
                default: None,
                required: true,
            },
            FieldSpec::Select {
                key: "quality",
                label: "压缩质量".into(),
                options: QUALITY_OPTIONS
                    .iter()
                    .map(|(v, l)| (v.to_string(), l.to_string()))
                    .collect(),
                default: Some(1), // medium 最常用，与 CLI 一致
            },
            FieldSpec::Text {
                key: "output",
                label: "输出路径（默认同目录换 .webp）".into(),
                placeholder: None,
                default: None,
                required: false,
            },
            FieldSpec::Toggle {
                key: "overwrite",
                label: "覆盖已存在文件".into(),
                default: false,
            },
        ]
    }

    fn resolve_options(&self, _key: &str, _values: &FieldValues) -> Option<Vec<(String, String)>> {
        None // 静态选项
    }

    fn build_task(&self, values: &FieldValues) -> Box<dyn TuiTask> {
        Box::new(CompressTask {
            input: values.text("file").unwrap_or_default().trim().to_string(),
            quality: values
                .choice("quality")
                .and_then(|s| s.parse::<u8>().ok())
                .unwrap_or_else(|| QualityPreset::Medium.value()),
            output: values
                .text("output")
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(PathBuf::from),
            overwrite: values.flag("overwrite").unwrap_or(false),
        })
    }
}

/// 质量选项：值 = cwebp -q 数值，展示 = 预设标签（与 CLI 交互一致）
const QUALITY_OPTIONS: [(&str, &str); 3] = [("85", "high (85)"), ("75", "medium (75)"), ("60", "low (60)")];

pub struct CompressTask {
    input: String,
    quality: u8,
    output: Option<PathBuf>,
    overwrite: bool,
}

impl TuiTask for CompressTask {
    /// 与 cli::compress::run_compress 行为一致，进度走 ctx.log
    ///（service 层的 indicatif spinner 会污染 TUI 备用屏幕，故直接调用 run_cwebp）。
    fn run(self: Box<Self>, ctx: TaskCtx) -> Result<TaskOutcome, String> {
        use crate::services::compress::format;

        let input = PathBuf::from(&self.input);
        if !input.exists() {
            return Err(format!("文件不存在: {}", input.display()));
        }
        let mut head = [0u8; 16];
        {
            use std::io::Read;
            let mut f = std::fs::File::open(&input).map_err(|e| e.to_string())?;
            let _ = f.read(&mut head);
        }
        let _fmt = format::detect_format(&input, &head).map_err(|e| e.to_string())?;
        ctx.log("格式校验通过（JPEG/PNG/WebP）");

        let out_path = self
            .output
            .clone()
            .unwrap_or_else(|| default_compress_output_path(&input));
        if out_path.exists() && !self.overwrite {
            return Err(format!(
                "输出文件已存在: {}（勾选「覆盖已存在文件」后重试）",
                out_path.display()
            ));
        }

        if ctx.is_cancelled() {
            return Err("已取消".into());
        }
        let original_size = std::fs::metadata(&input)
            .map_err(|e| e.to_string())?
            .len();
        ctx.log(format!("压缩为 WebP (q={}) → {}", self.quality, out_path.display()));
        webp::run_cwebp(&input, &out_path, self.quality).map_err(|e| e.to_string())?;
        let compressed_size = std::fs::metadata(&out_path)
            .map_err(|e| e.to_string())?
            .len();

        Ok(TaskOutcome::Success {
            summary: format!("已生成: {}", out_path.display()),
            details: vec![format!(
                "原始: {} → 压缩后: {} (节省 {}%)",
                format_size(original_size),
                format_size(compressed_size),
                savings_percent(original_size, compressed_size)
            )],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// PNG magic bytes 夹具（压缩入口仅校验格式，不解析内容）
    fn png_fixture(dir: &std::path::Path) -> std::path::PathBuf {
        let p = dir.join("photo.png");
        let mut head = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        head.extend_from_slice(&vec![0u8; 64]);
        std::fs::write(&p, head).unwrap();
        p
    }

    #[test]
    fn fields_cover_params() {
        let keys: Vec<&str> = CompressAction.fields().iter().map(|f| f.key()).collect();
        assert_eq!(keys, vec!["file", "quality", "output", "overwrite"]);
    }

    #[test]
    fn quality_options_match_presets() {
        let fields = CompressAction.fields();
        let opts = fields
            .iter()
            .find_map(|f| match f {
                FieldSpec::Select { key: "quality", options, .. } => Some(options.clone()),
                _ => None,
            })
            .unwrap();
        assert_eq!(opts.len(), 3);
        assert_eq!(opts[0], ("85".into(), "high (85)".into()));
        assert_eq!(opts[1], ("75".into(), "medium (75)".into()));
        assert_eq!(opts[2], ("60".into(), "low (60)".into()));
    }

    #[test]
    fn task_missing_input_fails() {
        let t = CompressTask {
            input: "/no/such.png".into(),
            quality: 75,
            output: None,
            overwrite: false,
        };
        let err = Box::new(t).run(TaskCtx::for_test()).unwrap_err();
        assert!(err.contains("文件不存在"));
    }

    #[test]
    fn task_unsupported_format_fails() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("photo.txt");
        std::fs::write(&p, b"not an image").unwrap();
        let t = CompressTask {
            input: p.to_string_lossy().into_owned(),
            quality: 75,
            output: None,
            overwrite: false,
        };
        let err = Box::new(t).run(TaskCtx::for_test()).unwrap_err();
        assert!(err.contains("不支持的压缩格式"));
    }

    #[test]
    fn task_output_conflict_without_overwrite() {
        let dir = tempfile::tempdir().unwrap();
        let input = png_fixture(dir.path());
        let out = dir.path().join("photo.webp");
        std::fs::write(&out, b"x").unwrap();
        let t = CompressTask {
            input: input.to_string_lossy().into_owned(),
            quality: 75,
            output: Some(out),
            overwrite: false,
        };
        let err = Box::new(t).run(TaskCtx::for_test()).unwrap_err();
        assert!(err.contains("输出文件已存在"));
    }
}
