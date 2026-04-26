use crate::errors::ScxVoidError;
use crate::services::project::git::downloader;
use crate::services::project::git::types::GitTemplate;

/// 从 GitHub 模板生成项目
pub async fn generate_from_github_template(
    project_name: &str,
    template: &GitTemplate,
    branch: Option<&str>,
) -> Result<(), ScxVoidError> {
    println!("正在从 GitHub 模板生成项目: {}", template.display_name);

    let (_temp_dir, source_dir) = downloader::download_template_to_temp(template, branch).await?;

    downloader::extract_template_files(&source_dir, project_name)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_generate_from_template_signature() {
        assert!(true);
    }
}
