#[derive(Debug)]
pub enum ScxVoidError {
    GeneralError(String),
    InvalidProjectName(String),
    ProjectAlreadyExists(String),
    FileSystemError(String),
    TemplateNotFound(String),
    UnsupportedProjectType(usize),
}

impl std::fmt::Display for ScxVoidError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ScxVoidError::GeneralError(msg) => write!(f, "Error: {}", msg),
            ScxVoidError::InvalidProjectName(msg) => write!(f, "Invalid project name: {}", msg),
            ScxVoidError::ProjectAlreadyExists(name) => write!(f, "Project already exists: {}", name),
            ScxVoidError::FileSystemError(msg) => write!(f, "File system error: {}", msg),
            ScxVoidError::TemplateNotFound(name) => write!(f, "Template not found: {}", name),
            ScxVoidError::UnsupportedProjectType(index) => write!(f, "Unsupported project type index: {}", index),
        }
    }
}

impl std::error::Error for ScxVoidError {}