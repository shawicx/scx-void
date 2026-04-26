use crate::errors::ScxVoidError;
use crate::services::project::git::archive;
use crate::services::project::git::types::GitTemplate;
use crate::utils::{fs, git};
use std::path::PathBuf;

/// 下载 GitHub 模板归档到临时目录
///
/// 返回 (TempDir, 内容目录 PathBuf)
pub async fn download_template_to_temp(
    template: &GitTemplate,
    branch: Option<&str>,
) -> Result<(tempfile::TempDir, PathBuf), ScxVoidError> {
    let temp_dir = tempfile::tempdir()
        .map_err(|e| ScxVoidError::FileSystemError(format!("创建临时目录失败: {}", e)))?;

    let (owner, repo) = git::parse_github_url(&template.repository_url)?;

    let branch = branch
        .map(String::from)
        .or_else(|| {
            if template.default_branch != "main" && !template.default_branch.is_empty() {
                Some(template.default_branch.clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "main".to_string());

    let extracted_dir =
        archive::download_and_extract_archive(&owner, &repo, &branch, temp_dir.path()).await?;

    // 如果模板指定了子路径，追加到解压目录
    let content_dir = if !template.template_path.is_empty() {
        let path = extracted_dir.join(template.template_path.trim_start_matches('/'));
        if !path.exists() {
            return Err(ScxVoidError::TemplateDownloadFailed(format!(
                "模板路径不存在: {}",
                template.template_path
            )));
        }
        path
    } else {
        extracted_dir
    };

    Ok((temp_dir, content_dir))
}

/// 将源目录内容复制到项目目录
pub fn extract_template_files(
    source_dir: &std::path::Path,
    project_name: &str,
) -> Result<(), ScxVoidError> {
    if !source_dir.exists() {
        return Err(ScxVoidError::TemplateDownloadFailed(format!(
            "源目录不存在: {:?}",
            source_dir
        )));
    }

    fs::copy_dir_all(source_dir.to_str().unwrap(), project_name)
        .map_err(|e| ScxVoidError::FileSystemError(format!("复制模板文件失败: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::project::git::types::ProjectType;

    #[test]
    fn test_extract_template_files() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source = temp_dir.path().join("test_file.txt");
        fs::write_file(source.to_str().unwrap(), "test content".to_string()).unwrap();

        let project_name = "test_project";

        let result = extract_template_files(temp_dir.path(), project_name);

        assert!(result.is_ok());
        assert!(std::path::Path::new(project_name).exists());

        std::fs::remove_dir_all(project_name).ok();
    }
}
