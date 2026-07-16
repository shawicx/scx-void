use assert_cmd::Command;
use std::fs;

#[allow(deprecated)]
fn create_cmd() -> Command {
    Command::cargo_bin("scx-void").unwrap()
}

/// convert 子命令应被注册并可被 --help 调用
#[test]
fn test_convert_command_registered() {
    let mut cmd = create_cmd();
    cmd.args(&["convert", "--help"]);
    cmd.assert().success();
}

/// 不存在的文件应失败退出
#[test]
fn test_convert_nonexistent_file_fails() {
    let mut cmd = create_cmd();
    cmd.args(&["convert", "/path/that/does/not/exist.heic"]);
    cmd.assert().failure();
}

/// 不支持的扩展名应失败退出（用一个真实存在的 .png 文件做输入）
#[test]
fn test_convert_unsupported_format_fails() {
    let tmp = tempfile_file("hello.png", b"not a real png");
    let mut cmd = create_cmd();
    cmd.args(&["convert", tmp.to_str().unwrap()]);
    let assert = cmd.assert();
    assert.failure();
    let _ = fs::remove_file(&tmp);
}

/// 扩展名为 heic 但内容不符应失败退出
#[test]
fn test_convert_magic_mismatch_fails() {
    let tmp = tempfile_file("fake.heic", b"this is not a heic file at all");
    let mut cmd = create_cmd();
    cmd.args(&["convert", tmp.to_str().unwrap()]);
    cmd.assert().failure();
    let _ = fs::remove_file(&tmp);
}

/// 不支持的目标格式应失败退出
#[test]
fn test_convert_unsupported_target_fails() {
    // 构造一个合法 HEIC 文件头，但请求转 webp（首期不支持）
    let mut head = vec![0u8; 64];
    head[4..8].copy_from_slice(b"ftyp");
    head[8..12].copy_from_slice(b"heic");
    let tmp = tempfile_file("real.heic", &head);
    let mut cmd = create_cmd();
    cmd.args(&["convert", tmp.to_str().unwrap(), "-f", "webp"]);
    cmd.assert().failure();
    let _ = fs::remove_file(&tmp);
}

/// 创建临时文件并写入内容，返回路径。
fn tempfile_file(name: &str, content: &[u8]) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(name);
    fs::write(&path, content).unwrap();
    path
}
