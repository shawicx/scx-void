// 模板下载器实现
// 将在 Phase 2 实现

use crate::errors::ScxVoidError;
use crate::services::project::git::types::GitTemplate;

/// 下载 Git 模板到临时目录
pub async fn download_template_to_temp(
    template: &GitTemplate,
    branch: Option<&str>,
) -> Result<tempfile::TempDir, ScxVoidError> {
    // TODO: Phase 2 实现
    Err(ScxVoidError::GeneralError(
        "模板下载功能将在 Phase 2 实现".to_string(),
    ))
}

/// 从临时目录提取模板文件到项目目录
pub fn extract_template_files(
    _temp_dir: &tempfile::TempDir,
    _template: &GitTemplate,
    _project_name: &str,
) -> Result<(), ScxVoidError> {
    // TODO: Phase 2 实现
    Err(ScxVoidError::GeneralError(
        "文件提取功能将在 Phase 2 实现".to_string(),
    ))
}
