pub mod ai_rule;
pub mod generator;
pub mod git;
pub mod installers;
pub mod project_service;
pub mod templates;

pub async fn create_project(
    project_name: &str,
    project_type_index: usize,
) -> Result<(), crate::errors::ScxVoidError> {
    project_service::create_project(project_name, project_type_index).await
}
