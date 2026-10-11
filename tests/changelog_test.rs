use assert_cmd::Command;
use std::fs;
use std::path::Path;
use std::process::Command as StdCommand;
use tempfile::TempDir;

fn create_cmd() -> Command {
    Command::cargo_bin("scx-void").unwrap()
}

fn setup_git_identity() {
    std::env::set_var("GIT_AUTHOR_NAME", "test");
    std::env::set_var("GIT_AUTHOR_EMAIL", "test@example.com");
    std::env::set_var("GIT_COMMITTER_NAME", "test");
    std::env::set_var("GIT_COMMITTER_EMAIL", "test@example.com");
}

fn run_git(dir: &Path, args: &[&str]) {
    setup_git_identity();
    let out = StdCommand::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&out.stderr)
    );
}

fn commit(dir: &Path, message: &str) {
    run_git(dir, &["add", "."]);
    run_git(dir, &["commit", "--allow-empty", "-m", message]);
}

fn make_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join(".gitkeep"), "").unwrap();
    run_git(tmp.path(), &["init"]);
    run_git(tmp.path(), &["add", "."]);
    run_git(tmp.path(), &["commit", "-m", "init"]);
    tmp
}

#[test]
fn test_changelog_init_orders_newest_first_and_splits_by_tags() {
    let tmp = make_repo();
    // v1.2.0 之后才有的提交，验证 v1.10.0 排在 v1.9.0 前面
    commit(tmp.path(), "feat: add a feature");
    run_git(tmp.path(), &["tag", "v1.2.0"]);
    commit(tmp.path(), "fix: bug one");
    run_git(tmp.path(), &["tag", "v1.9.0"]);
    commit(tmp.path(), "fix: bug two");
    run_git(tmp.path(), &["tag", "v1.10.0"]);
    commit(tmp.path(), "feat: unreleased feature");

    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["changelog", "init"])
        .output()
        .unwrap();
    assert!(output.status.success(), "init 失败");

    let content = fs::read_to_string(tmp.path().join("CHANGELOG.md")).unwrap();
    let pos_110 = content.find("## [v1.10.0]").expect("缺少 v1.10.0 段落");
    let pos_19 = content.find("## [v1.9.0]").expect("缺少 v1.9.0 段落");
    let pos_12 = content.find("## [v1.2.0]").expect("缺少 v1.2.0 段落");
    let pos_unreleased = content.find("## [Unreleased]").expect("缺少 Unreleased 段落");

    // 新版本在前：Unreleased > v1.10.0 > v1.9.0 > v1.2.0
    assert!(
        pos_unreleased < pos_110 && pos_110 < pos_19 && pos_19 < pos_12,
        "版本顺序错误:\n{}",
        content
    );
    // 提交分组正确
    assert!(content.contains("### Features"));
    assert!(content.contains("### Bug Fixes"));
    assert!(content.contains("- fix: bug two"));
    assert!(!content[pos_110..pos_19].contains("bug one"), "v1.10.0 段落不应包含上一版本提交");
    assert!(content[pos_unreleased..].contains("feat: unreleased feature"));
    // 提交带短哈希
    assert!(content.contains("(`"), "条目应包含短哈希:\n{}", content);
}

#[test]
fn test_changelog_init_refuses_existing_file_unless_force() {
    let tmp = make_repo();
    fs::write(tmp.path().join("CHANGELOG.md"), "手动内容").unwrap();

    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["changelog", "init"])
        .output()
        .unwrap();
    assert!(!output.status.success(), "已存在时应拒绝");

    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["changelog", "init", "--force"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let content = fs::read_to_string(tmp.path().join("CHANGELOG.md")).unwrap();
    assert!(content.contains("# Changelog"), "force 后应重新生成:\n{}", content);
}

#[test]
fn test_changelog_update_is_idempotent_and_merges_unreleased() {
    let tmp = make_repo();
    commit(tmp.path(), "feat: first feature");
    run_git(tmp.path(), &["tag", "v0.1.0"]);
    let _ = fs::remove_file(tmp.path().join(".gitkeep"));

    // 首次生成
    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["changelog", "init"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let before = fs::read_to_string(tmp.path().join("CHANGELOG.md")).unwrap();

    // 无新提交时 update 应幂等
    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["changelog", "update"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("无需更新"));
    let after = fs::read_to_string(tmp.path().join("CHANGELOG.md")).unwrap();
    assert_eq!(before, after, "无新提交时文件不应变化");

    // 新增未发布提交后 update 应合并进 Unreleased
    commit(tmp.path(), "feat: new feature");
    commit(tmp.path(), "fix: new fix");
    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["changelog", "update"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let updated = fs::read_to_string(tmp.path().join("CHANGELOG.md")).unwrap();
    assert!(updated.contains("feat: new feature"), "应包含新提交:\n{}", updated);
    assert!(updated.contains("fix: new fix"));
    assert!(updated.contains("## [v0.1.0]"), "旧版本段落应保留");

    // 重复 update 不应重复写入
    let snapshot = updated.clone();
    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["changelog", "update"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let updated2 = fs::read_to_string(tmp.path().join("CHANGELOG.md")).unwrap();
    assert_eq!(snapshot, updated2, "重复 update 应幂等");
}

#[test]
fn test_changelog_update_archives_new_tag_and_keeps_manual_notes() {
    let tmp = make_repo();
    commit(tmp.path(), "feat: v1 feature");
    run_git(tmp.path(), &["tag", "v1.0.0"]);

    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["changelog", "init"])
        .output()
        .unwrap();
    assert!(output.status.success());

    // 手工在 Unreleased 里加一条备注 + 一个新提交
    let path = tmp.path().join("CHANGELOG.md");
    let content = fs::read_to_string(&path).unwrap();
    fs::write(&path, content + "- 手工备注：等待依赖升级\n").unwrap();
    commit(tmp.path(), "feat: pending feature");
    run_git(tmp.path(), &["tag", "v1.1.0"]);

    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["changelog", "update"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let updated = fs::read_to_string(&path).unwrap();
    let pos_110 = updated.find("## [v1.1.0]").expect("新 tag 应归档为段落");
    let pos_100 = updated.find("## [v1.0.0]").expect("旧版本段落应保留");
    assert!(pos_110 < pos_100, "新版本段落应在旧版本之前:\n{}", updated);
    let sec_110 = &updated[pos_110..pos_100];
    assert!(!sec_110.contains("feat: v1 feature"), "v1.1.0 不应包含 v1.0.0 的提交:\n{}", sec_110);
    assert!(sec_110.contains("feat: pending feature"), "v1.1.0 应包含新发布提交:\n{}", sec_110);
    assert!(updated.contains("手工备注"), "手工条目应保留:\n{}", updated);
    assert!(updated[..pos_110].contains("## [Unreleased]"), "应存在 Unreleased 段落:\n{}", updated);
    assert!(!updated[..pos_110].contains("feat: pending feature"), "已发布提交不应留在 Unreleased");
}

#[test]
fn test_changelog_init_dry_run_and_no_tags() {
    let tmp = make_repo();
    commit(tmp.path(), "feat: only one");

    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["changelog", "init", "--dry-run"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[dry-run]"));
    assert!(stdout.contains("feat: only one"));
    assert!(!tmp.path().join("CHANGELOG.md").exists(), "dry-run 不应写文件");

    // 无 tag 时所有提交进 Unreleased
    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["changelog", "init"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let content = fs::read_to_string(tmp.path().join("CHANGELOG.md")).unwrap();
    assert!(content.contains("## [Unreleased]"));
    assert!(content.contains("feat: only one"));
}

#[test]
fn test_changelog_marks_breaking_changes() {
    let tmp = make_repo();
    commit(tmp.path(), "feat(api)!: drop legacy endpoint");
    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["changelog", "init"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let content = fs::read_to_string(tmp.path().join("CHANGELOG.md")).unwrap();
    assert!(content.contains("BREAKING CHANGES"), "应标记破坏性变更:\n{}", content);
    assert!(content.contains("feat(api)!: drop legacy endpoint"));
}

#[test]
fn test_changelog_fails_outside_git_repo() {
    let tmp = TempDir::new().unwrap();
    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["changelog", "init"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}
