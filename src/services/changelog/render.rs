use super::git::Commit;
use super::parse::{parse_subject, CommitKind};
use std::fmt::Write;

/// 分组渲染顺序
pub const GROUP_ORDER: [CommitKind; 7] = [
    CommitKind::Feat,
    CommitKind::Fix,
    CommitKind::Perf,
    CommitKind::Revert,
    CommitKind::Docs,
    CommitKind::Refactor,
    CommitKind::Other,
];

/// CHANGELOG 文件头（新版本在前，倒序排列）
pub fn header() -> String {
    "# Changelog\n\n本文件基于 git tag / 提交记录自动生成，最新版本在前。\n".to_string()
}

/// 把一组提交渲染为 `## [title] - date` 段落（按 Conventional Commits 分组）。
/// 无提交时返回 None。
pub fn render_section(title: &str, date: Option<&str>, commits: &[Commit]) -> Option<String> {
    if commits.is_empty() {
        return None;
    }
    let mut out = String::new();
    match date {
        Some(d) => {
            let _ = writeln!(out, "## [{}] - {}", title, d);
        }
        None => {
            let _ = writeln!(out, "## [{}]", title);
        }
    }
    // 破坏性变更条目单独前置
    let breaking: Vec<&Commit> = commits
        .iter()
        .filter(|c| parse_subject(&c.subject).breaking)
        .collect();
    if !breaking.is_empty() {
        out.push_str("### ⚠ BREAKING CHANGES\n\n");
        for c in &breaking {
            let _ = writeln!(out, "- {} (`{}`)", c.subject, c.short);
        }
        out.push('\n');
    }
    for kind in GROUP_ORDER {
        let group: Vec<&Commit> = commits
            .iter()
            .filter(|c| parse_subject(&c.subject).kind == kind)
            .collect();
        if group.is_empty() {
            continue;
        }
        let _ = writeln!(out, "### {}\n", kind.heading());
        for c in group {
            let _ = writeln!(out, "- {} (`{}`)", c.subject, c.short);
        }
        out.push('\n');
    }
    Some(out)
}
