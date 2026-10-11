/// 解析 Conventional Commits 风格的提交主题，归入对应的 CHANGELOG 分组
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitKind {
    Feat,
    Fix,
    Perf,
    Revert,
    Docs,
    Refactor,
    Other,
}

impl CommitKind {
    /// CHANGELOG 中的分组标题（渲染顺序由 render::GROUP_ORDER 决定）
    pub fn heading(self) -> &'static str {
        match self {
            Self::Feat => "Features",
            Self::Fix => "Bug Fixes",
            Self::Perf => "Performance Improvements",
            Self::Revert => "Reverts",
            Self::Docs => "Documentation",
            Self::Refactor => "Code Refactoring",
            Self::Other => "Other Changes",
        }
    }
}

/// 解析结果：提交类型 + 是否破坏性变更
#[derive(Debug, Clone, Copy)]
pub struct ParsedSubject {
    pub kind: CommitKind,
    pub breaking: bool,
}

/// 解析形如 `feat(scope)!: subject`、`fix: subject`、`chore: ...` 的主题。
/// 无法识别时归入 Other。`!` 或 `BREAKING CHANGE` 标记破坏性变更。
pub fn parse_subject(subject: &str) -> ParsedSubject {
    let breaking = subject.contains("!:"); // `type(scope)!: ...`
    let head = subject.split(':').next().unwrap_or("").trim();
    let head = head.trim_end_matches('!');
    let kind = match head.split('(').next().unwrap_or("").trim() {
        "feat" => CommitKind::Feat,
        "fix" => CommitKind::Fix,
        "perf" => CommitKind::Perf,
        "revert" => CommitKind::Revert,
        "docs" => CommitKind::Docs,
        "refactor" => CommitKind::Refactor,
        _ => CommitKind::Other,
    };
    ParsedSubject { kind, breaking }
}
