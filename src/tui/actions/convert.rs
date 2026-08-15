use crate::tui::actions::{
    ActionGroup, ActionMeta, FieldSpec, FieldValues, TuiAction, TaskCtx, TaskOutcome, TuiTask,
};
use std::path::PathBuf;

pub struct ConvertAction;

impl TuiAction for ConvertAction {
    fn meta(&self) -> ActionMeta {
        ActionMeta {
            id: "convert",
            title: "格式转换",
            description: "检测源文件格式并转换（如 HEIC → PNG）".into(),
            group: ActionGroup::File,
        }
    }

    fn fields(&self) -> Vec<FieldSpec> {
        vec![
            FieldSpec::Text {
                key: "file",
                label: "输入文件路径".into(),
                placeholder: Some("photo.heic".into()),
                default: None,
                required: true,
            },
            FieldSpec::Select {
                key: "format",
                label: "目标格式".into(),
                options: vec![], // 动态：聚焦时按输入文件解析
                default: None,
            },
            FieldSpec::Text {
                key: "output",
                label: "输出路径（默认同目录换后缀）".into(),
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

    fn resolve_options(&self, key: &str, values: &FieldValues) -> Option<Vec<(String, String)>> {
        if key != "format" {
            return None;
        }
        let path = PathBuf::from(values.text("file")?.trim());
        if !path.is_file() {
            return Some(vec![]);
        }
        let mut head = [0u8; 32];
        let ok = std::fs::File::open(&path)
            .and_then(|mut f| std::io::Read::read_exact(&mut f, &mut head))
            .is_ok();
        if !ok {
            return Some(vec![]);
        }
        let source = match crate::services::convert::registry::detect_format(&path, &head) {
            Ok(s) => s,
            Err(_) => return Some(vec![]),
        };
        Some(
            crate::services::convert::registry::target_formats(source)
                .into_iter()
                .map(|t| (t.to_string(), t.to_uppercase()))
                .collect(),
        )
    }

    fn build_task(&self, values: &FieldValues) -> Box<dyn TuiTask> {
        Box::new(ConvertTask {
            input: values.text("file").unwrap_or_default().trim().to_string(),
            format: values.choice("format").unwrap_or_default().to_string(),
            output: values
                .text("output")
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(PathBuf::from),
            overwrite: values.flag("overwrite").unwrap_or(false),
        })
    }
}

pub struct ConvertTask {
    input: String,
    format: String,
    output: Option<PathBuf>,
    overwrite: bool,
}

impl TuiTask for ConvertTask {
    fn run(self: Box<Self>, ctx: TaskCtx) -> Result<TaskOutcome, String> {
        use crate::services::convert::default_output_path;
        use crate::services::convert::registry;

        let input = PathBuf::from(&self.input);
        if !input.exists() {
            return Err(format!("文件不存在: {}", input.display()));
        }
        let mut head = [0u8; 32];
        {
            use std::io::Read;
            let mut f = std::fs::File::open(&input).map_err(|e| e.to_string())?;
            let _ = f.read(&mut head); // 文件可能不足 32 字节，忽略已读字节数
        }
        let source = registry::detect_format(&input, &head).map_err(|e| e.to_string())?;
        ctx.log(format!("检测到格式: {}", source.as_str().to_uppercase()));

        if !registry::is_supported_target(source, &self.format) {
            return Err(format!(
                "不支持的目标格式 '{}'，可选: {}",
                self.format,
                registry::target_formats(source).join(", ")
            ));
        }

        let out_path = self
            .output
            .clone()
            .unwrap_or_else(|| default_output_path(&input, &self.format));
        if out_path.exists() && !self.overwrite {
            return Err(format!(
                "输出文件已存在: {}（勾选「覆盖已存在文件」后重试）",
                out_path.display()
            ));
        }

        if ctx.is_cancelled() {
            return Err("已取消".into());
        }
        ctx.log(format!("正在转换 → {}", out_path.display()));
        registry::dispatch_convert(source, &input, &out_path, &self.format)
            .map_err(|e| e.to_string())?;
        Ok(TaskOutcome::Success {
            summary: format!("已生成: {}", out_path.display()),
            details: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::actions::FieldValue;

    fn heic_fixture(dir: &std::path::Path) -> std::path::PathBuf {
        let p = dir.join("photo.heic");
        let mut head = vec![0u8; 32];
        head[4..8].copy_from_slice(b"ftyp");
        head[8..12].copy_from_slice(b"heic");
        std::fs::write(&p, head).unwrap();
        p
    }

    #[test]
    fn fields_cover_all_params() {
        let keys: Vec<&str> = ConvertAction
            .fields()
            .iter()
            .map(|f| f.key())
            .collect();
        assert_eq!(keys, vec!["file", "format", "output", "overwrite"]);
    }

    #[test]
    fn resolve_options_from_valid_heic() {
        let dir = tempfile::tempdir().unwrap();
        let p = heic_fixture(dir.path());
        let mut v = FieldValues::default();
        v.0.insert(
            "file",
            FieldValue::Text(p.to_string_lossy().into_owned()),
        );
        let opts = ConvertAction.resolve_options("format", &v).unwrap();
        assert_eq!(opts, vec![("png".to_string(), "PNG".to_string())]);
    }

    #[test]
    fn resolve_options_missing_file_is_empty() {
        let mut v = FieldValues::default();
        v.0.insert("file", FieldValue::Text("/no/such/a.heic".into()));
        assert_eq!(
            ConvertAction.resolve_options("format", &v),
            Some(vec![])
        );
    }

    #[test]
    fn task_missing_input_fails() {
        let t = ConvertTask {
            input: "/no/such.heic".into(),
            format: "png".into(),
            output: None,
            overwrite: false,
        };
        let err = Box::new(t).run(TaskCtx::for_test()).unwrap_err();
        assert!(err.contains("文件不存在"));
    }

    #[test]
    fn task_output_conflict_without_overwrite() {
        let dir = tempfile::tempdir().unwrap();
        let input = heic_fixture(dir.path());
        let out = dir.path().join("photo.png");
        std::fs::write(&out, b"x").unwrap();
        let t = ConvertTask {
            input: input.to_string_lossy().into_owned(),
            format: "png".into(),
            output: Some(out),
            overwrite: false,
        };
        let err = Box::new(t).run(TaskCtx::for_test()).unwrap_err();
        assert!(err.contains("输出文件已存在"));
    }
}
