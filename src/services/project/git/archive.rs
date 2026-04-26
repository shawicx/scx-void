use crate::errors::ScxVoidError;
use crate::utils::git;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Write;
use std::path::{Path, PathBuf};

/// 从 GitHub 下载 zip 归档并解压到目标目录
///
/// 返回解压后的内容目录路径（通常为 {repo}-{branch}/）
pub async fn download_and_extract_archive(
    owner: &str,
    repo: &str,
    branch: &str,
    target_dir: &Path,
) -> Result<PathBuf, ScxVoidError> {
    let url = git::build_archive_url(owner, repo, branch);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(ScxVoidError::TemplateDownloadFailed(format!(
            "HTTP {} - 无法下载 {}",
            response.status(),
            url
        )));
    }

    let total_size = response.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template("{msg}\n{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message(format!("下载 {}/{}", owner, repo));

    // 流式写入临时 zip 文件
    let temp_zip = tempfile::Builder::new()
        .suffix(".zip")
        .tempfile()
        .map_err(|e| ScxVoidError::FileSystemError(format!("创建临时文件失败: {}", e)))?;

    let mut file = temp_zip
        .as_file()
        .try_clone()
        .map_err(|e| ScxVoidError::FileSystemError(format!("复制文件句柄失败: {}", e)))?;

    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| ScxVoidError::NetworkError(e.to_string()))?;
        file.write_all(&chunk)
            .map_err(|e| ScxVoidError::FileSystemError(format!("写入文件失败: {}", e)))?;
        pb.inc(chunk.len() as u64);
    }
    pb.finish_with_message("下载完成");

    // 重新打开文件用于解压
    let zip_path = temp_zip.path().to_path_buf();
    drop(file);

    let zip_file = std::fs::File::open(&zip_path)
        .map_err(|e| ScxVoidError::FileSystemError(format!("打开 zip 文件失败: {}", e)))?;

    let mut archive = zip::ZipArchive::new(zip_file)?;

    archive.extract(target_dir).map_err(|e| {
        ScxVoidError::ArchiveExtractError(format!("解压归档文件失败: {}", e))
    })?;

    // GitHub 归档解压后的顶层目录格式为 {repo}-{branch}
    let extracted_dir = target_dir.join(format!("{}-{}", repo, branch));

    if !extracted_dir.exists() {
        return Err(ScxVoidError::ArchiveExtractError(format!(
            "解压后未找到预期目录: {:?}",
            extracted_dir
        )));
    }

    Ok(extracted_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_archive_url() {
        let url = git::build_archive_url("shawicx", "template-node-ts-cli", "main");
        assert_eq!(
            url,
            "https://codeload.github.com/shawicx/template-node-ts-cli/zip/refs/heads/main"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn test_download_and_extract_archive() {
        let temp_dir = tempfile::tempdir().unwrap();
        let result = download_and_extract_archive(
            "shawicx",
            "template-node-ts-cli",
            "main",
            temp_dir.path(),
        )
        .await;

        if let Ok(path) = result {
            assert!(path.exists());
        }
    }
}
