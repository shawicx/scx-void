use assert_cmd::Command;

#[test]
fn tui_help_exits_zero() {
    Command::cargo_bin("scx-void")
        .unwrap()
        .arg("tui")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("tui"));
}

#[test]
fn tui_requires_tty() {
    // assert_cmd 通过管道捕获输出，stdout 不是 TTY，应立即报错退出
    Command::cargo_bin("scx-void")
        .unwrap()
        .arg("tui")
        .assert()
        .failure()
        .stderr(predicates::str::contains("TTY"));
}
