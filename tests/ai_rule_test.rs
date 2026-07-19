use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn create_cmd() -> Command {
    Command::cargo_bin("scx-void").unwrap()
}

/// 为每个测试创建独立的临时目录，避免并行测试互相污染进程级 CWD
/// （Rust 默认多线程跑测试，set_current_dir 是进程全局状态）
fn make_tempdir() -> TempDir {
    TempDir::new().unwrap()
}

#[test]
fn test_ai_rule_vue3_generates_file() {
    let tmp = make_tempdir();

    let mut cmd = create_cmd();
    cmd.current_dir(tmp.path());
    cmd.args(["project", "ai-rule", "-t", "vue3"]);
    cmd.assert().success();

    let agents = tmp.path().join("AGENTS.md");
    assert!(agents.exists());

    let content = fs::read_to_string(&agents).unwrap();
    // base 段
    assert!(content.contains("核心原则"));
    // Vue3 段
    assert!(content.contains("Vue 3 项目规则"));
    assert!(content.contains("pnpm"));
    assert!(content.contains("22+")); // Node.js 版本要求 22+
}

#[test]
fn test_ai_rule_unknown_stack_fails() {
    let tmp = make_tempdir();

    let mut cmd = create_cmd();
    cmd.current_dir(tmp.path());
    cmd.args(["project", "ai-rule", "-t", "foo"]);
    let output = cmd.output().unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("未知的技术栈类型"));
}

#[test]
fn test_ai_rule_force_overwrites_existing_file() {
    let tmp = make_tempdir();
    let agents = tmp.path().join("AGENTS.md");

    // 先放一个已有的 AGENTS.md
    fs::write(&agents, "# Old content").unwrap();

    // 不带 -f 应当失败
    let mut cmd = create_cmd();
    cmd.current_dir(tmp.path());
    cmd.args(["project", "ai-rule", "-t", "vue3"]);
    let output = cmd.output().unwrap();
    assert!(!output.status.success());

    // 带 -f 应当成功，并创建备份
    let mut cmd = create_cmd();
    cmd.current_dir(tmp.path());
    cmd.args(["project", "ai-rule", "-t", "vue3", "-f"]);
    cmd.assert().success();

    let content = fs::read_to_string(&agents).unwrap();
    assert!(content.contains("Vue 3 项目规则"));
    assert!(!content.contains("Old content"));

    // 备份文件保留原始内容
    let backup = tmp.path().join("AGENTS.md.backup");
    assert!(backup.exists());
    assert_eq!(fs::read_to_string(&backup).unwrap(), "# Old content");
}

#[test]
fn test_ai_rule_legacy_template_rejected() {
    let tmp = make_tempdir();

    // 旧的 -t basic / -t advanced 必须报错
    for legacy in &["basic", "advanced"] {
        let mut cmd = create_cmd();
        cmd.current_dir(tmp.path());
        cmd.args(["project", "ai-rule", "-t", legacy]);
        let output = cmd.output().unwrap();
        assert!(!output.status.success(), "-t {} 应当被拒绝", legacy);
    }
}

#[test]
fn test_ai_rule_all_stacks_generate() {
    let stacks = [
        "vue3", "react", "nextjs", "node-cli", "nestjs", "tauri", "java",
    ];

    for stack in &stacks {
        let tmp = make_tempdir();

        let mut cmd = create_cmd();
        cmd.current_dir(tmp.path());
        cmd.args(["project", "ai-rule", "-t", stack]);
        cmd.assert().success();

        let agents: PathBuf = tmp.path().join("AGENTS.md");
        assert!(agents.exists(), "stack={} 未生成文件", stack);
        let content = fs::read_to_string(&agents).unwrap();
        assert!(content.contains("核心原则"), "stack={} 缺少 base 段", stack);
    }
}
