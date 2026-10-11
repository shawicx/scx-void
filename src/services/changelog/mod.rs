pub mod git;
pub mod parse;
pub mod render;

use crate::errors::ScxVoidError;
use git::Commit;
use std::path::Path;

/// CHANGELOG 默认文件名
pub const CHANGELOG_FILE: &str = "CHANGELOG.md";

/// 生成的整体参数
pub struct ChangelogContext {
    pub file: String,
    pub dry_run: bool,
}

/// init：全量生成 CHANGELOG。已存在时拒绝（需手动删除或换 --file）
pub fn init(cwd: &Path, ctx: &ChangelogContext, force: bool) -> Result<(), ScxVoidError> {
    let path = cwd.join(&ctx.file);
    if path.exists() && !force {
        return Err(ScxVoidError::GeneralError(format!(
            "{} 已存在。如需重新生成请先删除，或使用 --file 指定其他输出文件",
            ctx.file
        )));
    }
    let content = build_full(cwd)?;
    write_out(&path, &content, ctx.dry_run, ctx.file.as_str())?;
    Ok(())
}

/// update：把新增 tag 段落与 未发布提交 合并进现有 CHANGELOG（幂等）。
/// 文件不存在时等价于 init。
pub fn update(cwd: &Path, ctx: &ChangelogContext) -> Result<(), ScxVoidError> {
    let path = cwd.join(&ctx.file);
    if !path.exists() {
        return init(cwd, ctx, false);
    }
    let existing = std::fs::read_to_string(&path)
        .map_err(|e| ScxVoidError::FileSystemError(format!("读取 {} 失败: {}", ctx.file, e)))?;

    let tags = git::version_tags(cwd)?;
    let (preamble, sections) = split_existing(&existing);

    // 1. 找出尚未写入 CHANGELOG 的版本 tag（新发布的版本，按版本从新到旧插入顶部）
    let mut new_sections: Vec<String> = Vec::new();
    let mut unreleased_commits;
    // 最新 tag 之前的历史已发布；无 tag 时所有提交都算未发布
    let latest = tags.last().map(|(name, _)| name.clone());
    for (name, _) in tags.iter().rev() {
        if sections.iter().any(|(title, _)| title.contains(name)) {
            break; // 更早的 tag 必然已在文件中（段落倒序排列）
        }
        let prev = previous_tag(&tags, name);
        let range = match &prev {
            Some(p) => format!("{}..{}", p, name),
            None => name.clone(),
        };
        let commits = git::commits_in_range(cwd, Some(&range))?;
        if let Some(section) = render::render_section(name, Some(&git::tag_date(cwd, name)?), &commits) {
            new_sections.push(section);
        }
    }
    // 2. 未发布提交：最新 tag..HEAD
    let range = latest.as_ref().map(|t| format!("{}..HEAD", t));
    unreleased_commits = git::commits_in_range(cwd, range.as_deref())?;

    // 3. 合并 Unreleased：保留旧文件中已有的条目（按短哈希去重），保留手工条目
    let old_unreleased = sections
        .iter()
        .find(|(title, _)| title.starts_with("[Unreleased]"))
        .map(|(_, body)| body.clone());
    let (old_commits, manual_lines) = split_unreleased_body(old_unreleased.as_deref());
    let seen: Vec<String> = old_commits.iter().map(|c| c.short.clone()).collect();
    unreleased_commits.retain(|c| !seen.contains(&c.short));
    let new_commit_count = unreleased_commits.len();
    unreleased_commits.extend(old_commits.iter().cloned());

    let mut merged = render::render_section("Unreleased", None, &unreleased_commits)
        .unwrap_or_else(|| "## [Unreleased]\n".to_string());
    if !manual_lines.is_empty() {
        if !merged.ends_with("\n\n") {
            merged.push('\n');
        }
        for line in &manual_lines {
            merged.push_str(line);
            merged.push('\n');
        }
    }

    // 4. 重组：preamble + Unreleased + 新版本段落 + 既有版本段落
    let mut out = String::new();
    out.push_str(preamble.trim_end());
    out.push_str("\n\n");
    out.push_str(&merged);
    for s in &new_sections {
        out.push_str(s);
    }
    for (title, body) in &sections {
        if title.starts_with("[Unreleased]") {
            continue;
        }
        out.push_str(&format!("## {}\n", title));
        out.push_str(body);
    }

    // 幂等检查：无新增版本段落、无新增未发布提交时提示并跳过写入
    if new_sections.is_empty() && new_commit_count == 0 {
        println!("没有新的提交或版本 tag，CHANGELOG 无需更新");
        return Ok(());
    }
    write_out(&path, &out, ctx.dry_run, ctx.file.as_str())?;
    Ok(())
}

/// 全量构建完整 CHANGELOG 内容（新版本在前）
fn build_full(cwd: &Path) -> Result<String, ScxVoidError> {
    let tags = git::version_tags(cwd)?;
    let mut sections: Vec<String> = Vec::new();

    // 未发布段落：最新 tag..HEAD；无 tag 时为全部历史
    let range = tags.last().map(|(name, _)| format!("{}..HEAD", name));
    let unreleased = git::commits_in_range(cwd, range.as_deref())?;
    if let Some(s) = render::render_section("Unreleased", None, &unreleased) {
        sections.push(s);
    }

    // 版本段落：从新到旧；最老的 tag 包含根提交
    for (i, (name, _)) in tags.iter().enumerate().rev() {
        let range = if i == 0 {
            name.clone()
        } else {
            format!("{}..{}", tags[i - 1].0, name)
        };
        let commits = git::commits_in_range(cwd, Some(&range))?;
        if let Some(s) = render::render_section(name, Some(&git::tag_date(cwd, name)?), &commits) {
            sections.push(s);
        }
    }

    let mut out = render::header().to_string();
    if sections.is_empty() {
        out.push_str("\n（暂无提交记录）\n");
    }
    for s in sections {
        out.push('\n');
        out.push_str(&s);
    }
    Ok(out)
}

/// 找到 tags 中位于 name 之前的那个 tag（即上一次发布）
fn previous_tag(tags: &[(String, semver::Version)], name: &str) -> Option<String> {
    let mut prev = None;
    for (n, _) in tags {
        if n == name {
            return prev;
        }
        prev = Some(n.clone());
    }
    None
}

/// 把现有 CHANGELOG 拆成 (头部说明, Vec<(段落标题(不含 ## ), 段落正文(含尾空行))>)
fn split_existing(content: &str) -> (String, Vec<(String, String)>) {
    let mut preamble = String::new();
    let mut sections: Vec<(String, String)> = Vec::new();
    let mut current: Option<(String, String)> = None;
    for line in content.lines() {
        if let Some(title) = line.strip_prefix("## ") {
            if let Some(c) = current.take() {
                sections.push(c);
            }
            current = Some((title.trim().to_string(), String::new()));
        } else if current.is_none() {
            preamble.push_str(line);
            preamble.push('\n');
        } else {
            current.as_mut().unwrap().1.push_str(line);
            current.as_mut().unwrap().1.push('\n');
        }
    }
    if let Some(c) = current.take() {
        sections.push(c);
    }
    (preamble, sections)
}

/// 从旧 Unreleased 段落正文中提取（已记录的提交, 手工条目行）。
/// 自动生成的条目形如 `- xxx (`abcdef1`)`。
fn split_unreleased_body(body: Option<&str>) -> (Vec<Commit>, Vec<String>) {
    let mut commits = Vec::new();
    let mut manual = Vec::new();
    let Some(body) = body else {
        return (commits, manual);
    };
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("### ") || trimmed.is_empty() {
            continue; // 分组标题与空行由渲染器重新生成
        }
        // 提取行内反引号中的短哈希
        let hash = trimmed
            .rsplit('`')
            .nth(1)
            .map(|s| s.to_string())
            .unwrap_or_default();
        let subject = trimmed
            .trim_start_matches("- ")
            .rsplit_once(" (`")
            .map(|(s, _)| s.to_string())
            .unwrap_or_else(|| trimmed.trim_start_matches("- ").to_string());
        if !hash.is_empty() && hash.len() >= 7 && hash.chars().all(|c| c.is_ascii_hexdigit()) {
            commits.push(Commit {
                short: hash,
                subject,
            });
        } else if !trimmed.is_empty() {
            manual.push(line.to_string());
        }
    }
    (commits, manual)
}

fn write_out(path: &Path, content: &str, dry_run: bool, display: &str) -> Result<(), ScxVoidError> {
    if dry_run {
        println!("[dry-run] 将写入 {}（{} 字节）:\n", display, content.len());
        println!("{}", content);
        return Ok(());
    }
    std::fs::write(path, content)
        .map_err(|e| ScxVoidError::FileSystemError(format!("写入 {} 失败: {}", display, e)))?;
    println!("已生成 {}", display);
    Ok(())
}
