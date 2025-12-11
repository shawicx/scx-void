pub mod installers;
pub mod templates;
pub mod generator;
pub mod project_service;
pub mod claude_rule;

pub use installers::*;
pub use templates::*;
pub use generator::*;
pub use project_service::*;
pub use claude_rule::*;

pub async fn create_project(project_name: &str, project_type_index: usize) -> Result<(), crate::errors::ScxVoidError> {
    project_service::create_project(project_name, project_type_index).await
}