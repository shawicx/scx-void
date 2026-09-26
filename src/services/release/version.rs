use crate::errors::ScxVoidError;
use semver::Version;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BumpKind {
    Patch,
    Minor,
    Major,
}

impl BumpKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Patch => "patch",
            Self::Minor => "minor",
            Self::Major => "major",
        }
    }
}

pub fn parse_version(s: &str) -> Result<Version, ScxVoidError> {
    Version::parse(s.trim())
        .map_err(|_| ScxVoidError::InvalidVersion(s.trim().to_string()))
}

pub fn bump(v: &Version, kind: BumpKind) -> Version {
    let (major, minor, patch) = match kind {
        BumpKind::Patch => (v.major, v.minor, v.patch + 1),
        BumpKind::Minor => (v.major, v.minor + 1, 0),
        BumpKind::Major => (v.major + 1, 0, 0),
    };
    Version::new(major, minor, patch)
}

fn read_json_file(path: &Path) -> Result<serde_json::Value, ScxVoidError> {
    let content = std::fs::read_to_string(path).map_err(|e| ScxVoidError::ManifestEditError {
        path: path.display().to_string(),
        reason: e.to_string(),
    })?;
    serde_json::from_str(&content).map_err(|e| ScxVoidError::ManifestEditError {
        path: path.display().to_string(),
        reason: e.to_string(),
    })
}

/// tauri.conf.json 的 version 字段为相对路径引用（如 "../package.json"）时，
/// 以 package.json 为版本源，无需修改 tauri.conf.json。
fn is_version_reference(v: &str) -> bool {
    v.ends_with(".json")
}

fn detect_indent(content: &str) -> usize {
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.len() < line.len() {
            let width = line.len() - trimmed.len();
            if width >= 4 {
                return 4;
            }
            if width >= 2 {
                return 2;
            }
        }
    }
    2
}

/// serde_json 固定输出 2 空格缩进，按原文件缩进宽度重排（4 空格时每两空格扩为四）
fn reindent(pretty: &str, indent: usize) -> String {
    if indent == 2 {
        return pretty.to_string();
    }
    pretty
        .lines()
        .map(|line| {
            let trimmed = line.trim_start_matches(' ');
            let depth = (line.len() - trimmed.len()) / 2;
            format!("{}{}", " ".repeat(depth * indent), trimmed)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn write_json_field(dir: &Path, file: &str, field: &str, value: &str) -> Result<(), ScxVoidError> {
    let path = dir.join(file);
    let content = std::fs::read_to_string(&path).map_err(|e| ScxVoidError::ManifestEditError {
        path: path.display().to_string(),
        reason: e.to_string(),
    })?;
    let indent = detect_indent(&content);
    let mut json: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| ScxVoidError::ManifestEditError {
            path: path.display().to_string(),
            reason: e.to_string(),
        })?;

    let Some(map) = json.as_object_mut() else {
        return Err(ScxVoidError::ManifestEditError {
            path: path.display().to_string(),
            reason: "根节点不是 JSON 对象".to_string(),
        });
    };
    map.insert(field.to_string(), serde_json::Value::String(value.to_string()));

    let pretty = serde_json::to_string_pretty(&json).map_err(|e| ScxVoidError::ManifestEditError {
        path: path.display().to_string(),
        reason: e.to_string(),
    })?;
    let mut out = reindent(&pretty, indent);
    if content.ends_with('\n') {
        out.push('\n');
    }
    std::fs::write(&path, out).map_err(|e| ScxVoidError::ManifestEditError {
        path: path.display().to_string(),
        reason: e.to_string(),
    })
}

pub fn read_package_version(root: &Path) -> Result<Version, ScxVoidError> {
    let path = root.join("package.json");
    let json = read_json_file(&path)?;
    let raw = json
        .get("version")
        .and_then(serde_json::Value::as_str)
        .ok_or(ScxVoidError::VersionFieldMissing(path))?;
    parse_version(raw)
}

pub fn package_is_private(root: &Path) -> bool {
    read_json_file(&root.join("package.json"))
        .ok()
        .and_then(|j| j.get("private").and_then(|v| v.as_bool()))
        .unwrap_or(false)
}

pub fn write_package_version(root: &Path, version: &str) -> Result<(), ScxVoidError> {
    write_json_field(root, "package.json", "version", version)
}

/// 返回 false 表示 tauri.conf.json 使用版本引用形式，未做修改
pub fn write_tauri_conf_version(src_tauri: &Path, version: &str) -> Result<bool, ScxVoidError> {
    let path = src_tauri.join("tauri.conf.json");
    let json = read_json_file(&path)?;
    if let Some(v) = json.get("version").and_then(|v| v.as_str()) {
        if is_version_reference(v) {
            return Ok(false);
        }
    }
    write_json_field(src_tauri, "tauri.conf.json", "version", version)?;
    Ok(true)
}

pub fn write_cargo_toml_version(src_tauri: &Path, version: &str) -> Result<(), ScxVoidError> {
    let path = src_tauri.join("Cargo.toml");
    let content = std::fs::read_to_string(&path).map_err(|e| ScxVoidError::ManifestEditError {
        path: path.display().to_string(),
        reason: e.to_string(),
    })?;
    let mut doc = content
        .parse::<toml_edit::DocumentMut>()
        .map_err(|e| ScxVoidError::ManifestEditError {
            path: path.display().to_string(),
            reason: e.to_string(),
        })?;

    let package = doc
        .get_mut("package")
        .and_then(|item| item.as_table_like_mut())
        .ok_or_else(|| ScxVoidError::ManifestEditError {
            path: path.display().to_string(),
            reason: "缺少 [package] 表".to_string(),
        })?;

    // 原位更新已有键（保留位置与装饰）；不存在则追加
    if let Some(item) = package.get_mut("version") {
        *item = toml_edit::value(version);
    } else {
        package.insert("version", toml_edit::value(version));
    }

    std::fs::write(&path, doc.to_string()).map_err(|e| ScxVoidError::ManifestEditError {
        path: path.display().to_string(),
        reason: e.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn write(path: &std::path::Path, content: &str) {
        std::fs::write(path, content).unwrap();
    }

    #[test]
    fn bump_kind_rules() {
        let v = parse_version("1.2.3").unwrap();
        assert_eq!(bump(&v, BumpKind::Patch).to_string(), "1.2.4");
        assert_eq!(bump(&v, BumpKind::Minor).to_string(), "1.3.0");
        assert_eq!(bump(&v, BumpKind::Major).to_string(), "2.0.0");
        assert_eq!(bump(&v, BumpKind::Patch).major, 1);
    }

    #[test]
    fn bump_clears_prerelease() {
        let v = parse_version("1.2.3-alpha.1").unwrap();
        assert_eq!(bump(&v, BumpKind::Patch).to_string(), "1.2.4");
        let v0 = parse_version("0.0.0").unwrap();
        assert_eq!(bump(&v0, BumpKind::Patch).to_string(), "0.0.1");
    }

    #[test]
    fn parse_rejects_invalid() {
        assert!(parse_version("abc").is_err());
        assert!(parse_version("1.2").is_err());
        assert!(parse_version(" 1.2.3 ").is_ok());
    }

    #[test]
    fn write_package_version_preserves_key_order_and_indent() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("package.json"),
            "{\n    \"name\": \"demo\",\n    \"version\": \"0.1.0\",\n    \"private\": false\n}\n",
        );

        write_package_version(tmp.path(), "0.2.0").unwrap();

        let out = std::fs::read_to_string(tmp.path().join("package.json")).unwrap();
        assert!(out.starts_with("{\n    \"name\": \"demo\""), "键序保持 name 在前:\n{}", out);
        assert!(out.contains("    \"version\": \"0.2.0\""), "4 空格缩进保持:\n{}", out);
        assert!(out.ends_with("}\n"));
        assert_eq!(read_package_version(tmp.path()).unwrap().to_string(), "0.2.0");
    }

    #[test]
    fn read_package_version_missing_field() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("package.json"),
            "{\n  \"name\": \"demo\"\n}\n",
        );
        assert!(matches!(
            read_package_version(tmp.path()),
            Err(ScxVoidError::VersionFieldMissing(_))
        ));
    }

    #[test]
    fn package_private_detection() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("package.json"),
            "{\n  \"name\": \"demo\",\n  \"version\": \"1.0.0\",\n  \"private\": true\n}\n",
        );
        assert!(package_is_private(tmp.path()));
    }

    #[test]
    fn tauri_conf_version_reference_is_skipped() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("tauri.conf.json"),
            "{\n  \"version\": \"../package.json\"\n}\n",
        );
        let updated = write_tauri_conf_version(tmp.path(), "1.1.0").unwrap();
        assert!(!updated, "引用形式应跳过");
        let out = std::fs::read_to_string(tmp.path().join("tauri.conf.json")).unwrap();
        assert!(out.contains("../package.json"));
    }

    #[test]
    fn tauri_conf_version_normal_is_written() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("tauri.conf.json"),
            "{\n  \"productName\": \"demo\",\n  \"version\": \"1.0.0\"\n}\n",
        );
        let updated = write_tauri_conf_version(tmp.path(), "1.1.0").unwrap();
        assert!(updated);
        let out = std::fs::read_to_string(tmp.path().join("tauri.conf.json")).unwrap();
        assert!(out.contains("\"version\": \"1.1.0\""));
    }

    #[test]
    fn cargo_toml_preserves_comments_and_position() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("Cargo.toml"),
            "[package]\nname = \"demo\"\n# 版本注释\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nserde = \"1\"\n",
        );

        write_cargo_toml_version(tmp.path(), "0.2.0").unwrap();

        let out = std::fs::read_to_string(tmp.path().join("Cargo.toml")).unwrap();
        assert!(out.contains("# 版本注释"), "注释保留:\n{}", out);
        assert!(out.contains("version = \"0.2.0\""));
        assert!(out.find("version = \"0.2.0\"").unwrap() < out.find("edition").unwrap());
        assert!(out.contains("serde = \"1\""));
    }

    #[test]
    fn cargo_toml_missing_package_table_fails() {
        let tmp = TempDir::new().unwrap();
        write(&tmp.path().join("Cargo.toml"), "[dependencies]\nserde = \"1\"\n");
        assert!(write_cargo_toml_version(tmp.path(), "0.2.0").is_err());
    }
}
