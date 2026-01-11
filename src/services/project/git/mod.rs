pub mod clone;
pub mod downloader;
pub mod registry;
pub mod types;
pub mod validator;

// 导出公共接口
pub use registry::{get_all_templates, get_template_by_id, get_template_map, template_exists};
pub use types::{CloneOptions, GitTemplate, ProjectType, TemplateSource};
pub use validator::{
    check_git_installed, is_valid_branch_name, validate_clone_options, validate_git_template,
    validate_template_id, validate_template_path,
};
