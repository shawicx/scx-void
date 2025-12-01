#[derive(Debug)]
pub enum ScxVoidError {
    GeneralError(String),
}

impl std::fmt::Display for ScxVoidError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ScxVoidError::GeneralError(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for ScxVoidError {}