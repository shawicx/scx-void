use assert_cmd::Command;
use std::fs;

#[allow(deprecated)]
fn create_cmd() -> Command {
    Command::cargo_bin("scx-void").unwrap()
}

/// compress 子命令应被注册并可被 --help 调用
#[test]
fn test_compress_command_registered() {
    let mut cmd = create_cmd();
    cmd.args(&["compress", "--help"]);
    cmd.assert().success();
}

/// 不存在的文件应失败退出
#[test]
fn test_compress_nonexistent_file_fails() {
    let mut cmd = create_cmd();
    cmd.args(&["compress", "/path/that/does/not/exist.jpg"]);
    cmd.assert().failure();
}

/// 不支持的格式（GIF 文件头）应失败退出
#[test]
fn test_compress_unsupported_format_fails() {
    let tmp = tempfile_file("bad.gif", b"GIF89a dummy content");
    let mut cmd = create_cmd();
    cmd.args(&["compress", tmp.to_str().unwrap()]);
    cmd.assert().failure();
    let _ = fs::remove_file(&tmp);
}

/// 质量越界（0）应失败退出
#[test]
fn test_compress_quality_zero_fails() {
    // 构造合法 PNG 头，让格式检测通过，但质量 0 越界
    let mut head = vec![0u8; 32];
    head[0..8].copy_from_slice(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
    let tmp = tempfile_file("ok.png", &head);
    let mut cmd = create_cmd();
    cmd.args(&["compress", tmp.to_str().unwrap(), "-q", "0"]);
    cmd.assert().failure();
    let _ = fs::remove_file(&tmp);
}

/// 质量越界（200）应失败退出
#[test]
fn test_compress_quality_over_100_fails() {
    let mut head = vec![0u8; 32];
    head[0..8].copy_from_slice(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
    let tmp = tempfile_file("ok2.png", &head);
    // clap u8 解析：200 在 u8 范围内，会被解析为 200，进入业务层范围校验失败
    let mut cmd = create_cmd();
    cmd.args(&["compress", tmp.to_str().unwrap(), "-q", "200"]);
    cmd.assert().failure();
    let _ = fs::remove_file(&tmp);
}

/// 创建临时文件并写入内容，返回路径。
fn tempfile_file(name: &str, content: &[u8]) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(name);
    fs::write(&path, content).unwrap();
    path
}
