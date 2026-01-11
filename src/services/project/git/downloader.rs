use crate::errors::ScxVoidError;
use crate::services::project::git::clone;
use crate::services::project::git::types::{CloneOptions, GitTemplate};
use crate::utils::fs;

/// 下载 Git 模板到临时目录
pub async fn download_template_to_temp(
    template: &GitTemplate,
    branch: Option<&str>,
) -> Result<tempfile::TempDir, ScxVoidError> {
    let temp_dir = tempfile::tempdir()
        .map_err(|e| ScxVoidError::FileSystemError(format!("创建临时目录失败: {}", e)))?;

    let target_dir = temp_dir
        .path()
        .to_str()
        .ok_or_else(|| ScxVoidError::FileSystemError("临时目录路径无效".to_string()))?;

    let clone_options =
        CloneOptions::new(template.repository_url.clone(), target_dir.to_string()).with_depth(1);

    if !template.template_path.is_empty() {
        clone::clone_sparse_checkout(clone_options, &template.template_path).await?;
    } else {
        let clone_options = if let Some(b) = branch.or_else(|| {
            if template.default_branch != "main" {
                Some(&template.default_branch)
            } else {
                None
            }
        }) {
            clone_options.with_branch(b.to_string())
        } else {
            clone_options
        };

        clone::clone_repository(clone_options).await?;
    }

    Ok(temp_dir)
}

/// 从临时目录提取模板文件到项目目录
pub fn extract_template_files(
    temp_dir: &tempfile::TempDir,
    template: &GitTemplate,
    project_name: &str,
) -> Result<(), ScxVoidError> {
    let source_dir = if !template.template_path.is_empty() {
        let path = temp_dir
            .path()
            .join(template.template_path.trim_start_matches('/'));
        if !path.exists() {
            return Err(ScxVoidError::TemplateDownloadFailed(format!(
                "模板路径不存在: {}",
                template.template_path
            )));
        }
        path
    } else {
        temp_dir.path().to_path_buf()
    };

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

    #[test]
    fn test_extract_template_files() {
        let template = GitTemplate::predefined(
            "test",
            "Test Template",
            "Test Description",
            "https://github.com/test/repo.git",
            crate::services::project::git::types::ProjectType::NodeTsCli,
        );

        let temp_dir = tempfile::tempdir().unwrap();
        let source = temp_dir.path().join("test_file.txt");
        fs::write_file(source.to_str().unwrap(), "test content".to_string()).unwrap();

        let project_name = "test_project";

        let result = extract_template_files(&temp_dir, &template, project_name);

        assert!(result.is_ok());
        assert!(std::path::Path::new(project_name).exists());

        std::fs::remove_dir_all(project_name).ok();
    }
}
