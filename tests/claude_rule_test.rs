use assert_cmd::Command;
use std::fs;
use std::path::Path;
use std::os::unix::prelude::OsStringExt;

#[test]
fn test_claude_rule_basic_template() {
    // 清理测试环境
    cleanup_test_files();

    let mut cmd = Command::cargo_bin("scx-void").unwrap();
    cmd.args(&["project", "claude-rule", "--template", "basic"]);

    let assert = cmd.assert();
    assert.success();

    // 检查文件是否生成
    assert!(Path::new(".claude-code-rule.md").exists());

    // 检查文件内容
    let content = fs::read_to_string(".claude-code-rule.md").unwrap();
    assert!(content.contains("AI Code Agent - Basic Project Rules"));
    assert!(content.contains("核心原则"));

    cleanup_test_files();
}

#[test]
fn test_claude_rule_advanced_template() {
    // 清理测试环境
    cleanup_test_files();

    let mut cmd = Command::cargo_bin("scx-void").unwrap();
    cmd.args(&["project", "claude-rule", "--template", "advanced"]);

    let assert = cmd.assert();
    assert.success();

    // 检查文件是否生成
    assert!(Path::new(".claude-code-rule.md").exists());

    // 检查文件内容
    let content = fs::read_to_string(".claude-code-rule.md").unwrap();
    assert!(content.contains("AI Code Agent - Multi-Project CLI Rules"));
    assert!(content.contains("禁止事项"));
    assert!(content.contains("兼容性要求"));

    cleanup_test_files();
}

#[test]
fn test_claude_rule_force_overwrite() {
    // 清理测试环境
    cleanup_test_files();

    // 首先创建一个文件
    fs::write(".claude-code-rule.md", "# Existing content").unwrap();

    // 尝试不使用 force 覆盖
    let mut cmd = Command::cargo_bin("scx-void").unwrap();
    cmd.args(&["project", "claude-rule", "--template", "basic"]);

    let output = cmd.output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("already exists"));

    // 使用 force 覆盖
    let mut cmd = Command::cargo_bin("scx-void").unwrap();
    cmd.args(&["project", "claude-rule", "--template", "basic", "--force"]);

    let assert = cmd.assert();
    assert.success();

    // 检查内容是否被覆盖
    let content = fs::read_to_string(".claude-code-rule.md").unwrap();
    assert!(content.contains("AI Code Agent - Basic Project Rules"));
    assert!(!content.contains("Existing content"));

    cleanup_test_files();
}

#[test]
fn test_claude_rule_invalid_template() {
    // 清理测试环境
    cleanup_test_files();

    let mut cmd = Command::cargo_bin("scx-void").unwrap();
    cmd.args(&["project", "claude-rule", "--template", "invalid"]);

    let output = cmd.output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid template type"));

    cleanup_test_files();
}

#[test]
fn test_claude_rule_backup_functionality() {
    // 清理测试环境
    cleanup_test_files();

    // 首先创建一个文件
    fs::write(".claude-code-rule.md", "# Original content").unwrap();

    // 使用 force 覆盖（应该创建备份）
    let mut cmd = Command::cargo_bin("scx-void").unwrap();
    cmd.args(&["project", "claude-rule", "--template", "basic", "--force"]);

    let assert = cmd.assert();
    assert.success();

    // 检查备份文件是否创建
    assert!(Path::new(".claude-code-rule.md.backup").exists());

    // 检查备份文件内容
    let backup_content = fs::read_to_string(".claude-code-rule.md.backup").unwrap();
    assert_eq!(backup_content, "# Original content");

    cleanup_test_files();
}

fn cleanup_test_files() {
    let files_to_remove = [
        ".claude-code-rule.md",
        ".claude-code-rule.md.backup",
    ];

    for file in &files_to_remove {
        if Path::new(file).exists() {
            let _ = fs::remove_file(file);
        }
    }
}