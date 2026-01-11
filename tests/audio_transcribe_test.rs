use assert_cmd::Command;

#[test]
fn test_audio_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scx-void")?;
    let assert = cmd.arg("audio").arg("--help").assert();
    assert
        .success()
        .stdout(predicates::str::contains("音频转录相关命令"));
    Ok(())
}

#[test]
fn test_audio_list_models() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scx-void")?;
    let assert = cmd.arg("audio").arg("list-models").assert();
    assert
        .success()
        .stdout(predicates::str::contains("可用的 Whisper 模型"))
        .stdout(predicates::str::contains("tiny"))
        .stdout(predicates::str::contains("base"))
        .stdout(predicates::str::contains("small"))
        .stdout(predicates::str::contains("medium"))
        .stdout(predicates::str::contains("large"));
    Ok(())
}

#[test]
fn test_transcribe_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scx-void")?;
    let assert = cmd.arg("audio").arg("transcribe").arg("--help").assert();
    assert
        .success()
        .stdout(predicates::str::contains("转录音频文件为文本"))
        .stdout(predicates::str::contains("音频文件路径"))
        .stdout(predicates::str::contains("语言代码"));
    Ok(())
}

#[test]
fn test_transcribe_with_timestamps_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scx-void")?;
    let assert = cmd
        .arg("audio")
        .arg("transcribe-with-timestamps")
        .arg("--help")
        .assert();
    assert.success().stdout(predicates::str::contains(
        "转录音频文件并生成带时间戳的文本",
    ));
    Ok(())
}

#[test]
fn test_download_model_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scx-void")?;
    let assert = cmd
        .arg("audio")
        .arg("download-model")
        .arg("--help")
        .assert();
    assert
        .success()
        .stdout(predicates::str::contains("下载 Whisper 模型"))
        .stdout(predicates::str::contains("模型大小"));
    Ok(())
}

#[test]
fn test_download_model_invalid_size() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scx-void")?;
    let assert = cmd
        .arg("audio")
        .arg("download-model")
        .arg("invalid")
        .assert();
    assert
        .failure()
        .stderr(predicates::str::contains("未知的模型大小"));
    Ok(())
}

#[test]
fn test_transcribe_missing_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scx-void")?;
    let assert = cmd
        .arg("audio")
        .arg("transcribe")
        .arg("nonexistent.m4a")
        .assert();
    assert
        .failure()
        .stderr(predicates::str::contains("音频文件不存在"));
    Ok(())
}

#[test]
fn test_transcribe_missing_model() -> Result<(), Box<dyn std::error::Error>> {
    // 创建一个临时的假音频文件
    let temp_file = "test_temp.m4a";
    std::fs::write(temp_file, "fake audio content")?;

    let mut cmd = Command::cargo_bin("scx-void")?;
    let assert = cmd.arg("audio").arg("transcribe").arg(temp_file).assert();
    assert
        .failure()
        .stderr(predicates::str::contains("未找到默认模型"));

    // 清理临时文件
    std::fs::remove_file(temp_file)?;
    Ok(())
}
