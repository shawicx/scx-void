use crate::errors::ScxVoidError;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectKind {
    Node,
    Tauri,
}

impl ProjectKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Node => "Node.js CLI",
            Self::Tauri => "Tauri V2",
        }
    }
}

pub fn detect(root: &Path, forced: Option<&str>) -> Result<ProjectKind, ScxVoidError> {
    if let Some(t) = forced {
        return match t.trim().to_ascii_lowercase().as_str() {
            "node" | "nodejs" => Ok(ProjectKind::Node),
            "tauri" => Ok(ProjectKind::Tauri),
            other => Err(ScxVoidError::ReleaseProjectNotDetected(format!(
                "--type '{}' 无效，可选: node, tauri",
                other
            ))),
        };
    }
    if root.join("src-tauri/tauri.conf.json").exists() {
        Ok(ProjectKind::Tauri)
    } else if root.join("package.json").exists() {
        Ok(ProjectKind::Node)
    } else {
        Err(ScxVoidError::ReleaseProjectNotDetected(
            "未找到 package.json 或 src-tauri/tauri.conf.json".to_string(),
        ))
    }
}
