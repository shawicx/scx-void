pub mod clone;
pub mod downloader;
pub mod registry;
pub mod types;
pub mod validator;

// 导出公共接口
pub use types::{ProjectType, GitTemplate, CloneOptions, TemplateSource};
pub use registry::{get_all_templates, get_template_by_id, template_exists, get_template_map};
pub use validator::{
    validate_git_template,
    is_valid_branch_name,
    validate_template_id,
    validate_template_path,
    validate_clone_options,
    check_git_installed,
};
