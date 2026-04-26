pub mod archive;
pub mod downloader;
pub mod registry;
pub mod types;
pub mod validator;

// 导出公共接口
#[allow(unused_imports)]
pub use registry::{get_all_templates, get_template_by_id, get_template_map, template_exists};
#[allow(unused_imports)]
pub use types::{GitTemplate, ProjectType};
#[allow(unused_imports)]
pub use validator::{is_valid_branch_name, validate_git_template, validate_template_id, validate_template_path};
