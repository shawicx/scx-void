use assert_cmd::Command;
use std::fs;
use std::path::Path;
use std::process::Command as StdCommand;
use tempfile::TempDir;

fn create_cmd() -> Command {
    Command::cargo_bin("scx-void").unwrap()
}

/// 为 release 内部执行的 `git commit` 提供身份（子进程继承环境变量，
/// 避免污染全局 git 配置；并行测试写入相同值是安全的）
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

/// 构造一个带 package.json 的干净 git 仓库
fn make_node_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("package.json"),
        "{\n  \"name\": \"demo-cli\",\n  \"version\": \"0.1.0\",\n  \"bin\": { \"demo\": \"./bin.js\" }\n}\n",
    )
    .unwrap();
    run_git(tmp.path(), &["init"]);
    run_git(tmp.path(), &["add", "."]);
    run_git(tmp.path(), &["commit", "-m", "init"]);
    tmp
}

#[test]
fn test_release_dry_run_prints_steps_without_changes() {
    let tmp = make_node_repo();

    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["release", "--patch", "--dry-run"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Node.js CLI"), "应检测出 Node 类型:\n{}", stdout);
    assert!(stdout.contains("0.1.0 → 0.1.1"), "应展示版本变化:\n{}", stdout);
    assert!(stdout.contains("git commit -m \"chore: release v0.1.1\""));
    assert!(stdout.contains("git tag v0.1.1"));
    assert!(stdout.contains("npm publish"));
    assert!(stdout.contains("[dry-run]"), "应有 dry-run 标识:\n{}", stdout);

    // 文件与 git 状态未被修改
    let pkg = fs::read_to_string(tmp.path().join("package.json")).unwrap();
    assert!(pkg.contains("\"version\": \"0.1.0\""));
    let tags = StdCommand::new("git")
        .args(["tag", "-l"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&tags.stdout).trim().is_empty());
}

#[test]
fn test_release_fails_outside_git_repo() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("package.json"),
        "{\n  \"name\": \"demo\",\n  \"version\": \"1.0.0\"\n}\n",
    )
    .unwrap();

    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["release", "--patch", "--yes"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("不是 git 仓库"));
}

#[test]
fn test_release_fails_on_dirty_worktree() {
    let tmp = make_node_repo();
    fs::write(tmp.path().join("dirty.txt"), "uncommitted").unwrap();

    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["release", "--patch", "--yes"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("未提交改动"));
}

#[test]
fn test_release_full_local_path_bumps_commits_and_tags() {
    let tmp = make_node_repo();

    let mut cmd = create_cmd();
    cmd.current_dir(tmp.path())
        .args([
            "release",
            "--minor",
            "--yes",
            "--no-push",
            "--no-publish",
        ])
        .assert()
        .success();
    let pkg = fs::read_to_string(tmp.path().join("package.json")).unwrap();
    assert!(pkg.contains("\"version\": \"0.2.0\""));
    // 键序保持
    assert!(pkg.find("\"name\"").unwrap() < pkg.find("\"version\"").unwrap());

    // commit 与 tag 已创建
    let log = StdCommand::new("git")
        .args(["log", "-1", "--pretty=%s"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(String::from_utf8_lossy(&log.stdout).trim(), "chore: release v0.2.0");

    let tags = StdCommand::new("git")
        .args(["tag", "-l"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(String::from_utf8_lossy(&tags.stdout).trim(), "v0.2.0");
}

#[test]
fn test_release_fails_when_tag_exists() {
    let tmp = make_node_repo();
    run_git(tmp.path(), &["tag", "v0.2.0"]);

    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["release", "--minor", "--yes", "--no-push", "--no-publish"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("v0.2.0' 已存在"));
}

#[test]
fn test_release_tauri_type_requires_src_tauri_files() {
    let tmp = make_node_repo();

    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["release", "--type", "tauri", "--patch", "--yes", "--dry-run"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("tauri.conf.json"));
}

#[test]
fn test_release_publish_and_no_publish_are_mutually_exclusive() {
    let tmp = make_node_repo();

    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["release", "--patch", "--dry-run", "--publish", "--no-publish"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("--publish"));
}

#[test]
fn test_release_no_publish_skips_npm_publish_step() {
    let tmp = make_node_repo();

    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["release", "--patch", "--dry-run", "--no-publish"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("npm publish"), "应无 npm publish 步骤:\n{}", stdout);
    assert!(stdout.contains("git tag v0.1.1"));
}

#[test]
fn test_release_private_package_allowed_when_no_publish() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("package.json"),
        "{\n  \"name\": \"demo\",\n  \"version\": \"1.0.0\",\n  \"private\": true\n}\n",
    )
    .unwrap();
    run_git(tmp.path(), &["init"]);
    run_git(tmp.path(), &["add", "."]);
    run_git(tmp.path(), &["commit", "-m", "init"]);

    // private:true 且不发布 npm（CI 发布场景）→ 放行
    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["release", "--patch", "--dry-run", "--no-publish", "--no-push"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("1.0.0 → 1.0.1"));

    // private:true 且要发布（dry-run 默认走发布分支）→ 仍拦截
    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["release", "--patch", "--dry-run"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("private"));
}

#[test]
fn test_release_tauri_dry_run_syncs_three_files_plan() {
    let tmp = make_node_repo();
    let src_tauri = tmp.path().join("src-tauri");
    fs::create_dir_all(&src_tauri).unwrap();
    fs::write(
        src_tauri.join("tauri.conf.json"),
        "{\n  \"productName\": \"demo\",\n  \"version\": \"0.1.0\"\n}\n",
    )
    .unwrap();
    fs::write(
        src_tauri.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    run_git(tmp.path(), &["add", "."]);
    run_git(tmp.path(), &["commit", "-m", "add tauri"]);

    let output = create_cmd()
        .current_dir(tmp.path())
        .args(["release", "--patch", "--dry-run", "--no-push"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Tauri V2"), "应检测出 Tauri 类型:\n{}", stdout);
    assert!(stdout.contains("同步更新 package.json / tauri.conf.json / Cargo.toml"));
    assert!(!stdout.contains("npm publish"), "Tauri 流程不应包含 npm publish");

    // dry-run 未改文件
    let conf = fs::read_to_string(src_tauri.join("tauri.conf.json")).unwrap();
    assert!(conf.contains("\"version\": \"0.1.0\""));
}
