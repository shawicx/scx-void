#[derive(Debug)]
#[allow(dead_code)]
pub enum ScxVoidError {
    GeneralError(String),
    InvalidProjectName(String),
    ProjectAlreadyExists(String),
    FileSystemError(String),
    TemplateNotFound(String),
    UnsupportedProjectType(usize),
    ClaudeRuleFileExists(std::path::PathBuf),
    InvalidTemplateType(String),
    AudioFileNotFound(String),
    UnsupportedAudioFormat(String),
    AudioDecodingError(String),
    WhisperModelNotFound(String),
    WhisperLoadError(String),
    TranscriptionError(String),
    ModelDownloadError(String),
    NetworkError(String),
}

impl std::fmt::Display for ScxVoidError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ScxVoidError::GeneralError(msg) => write!(f, "Error: {}", msg),
            ScxVoidError::InvalidProjectName(msg) => write!(f, "Invalid project name: {}", msg),
            ScxVoidError::ProjectAlreadyExists(name) => {
                write!(f, "Project already exists: {}", name)
            }
            ScxVoidError::FileSystemError(msg) => write!(f, "File system error: {}", msg),
            ScxVoidError::TemplateNotFound(name) => write!(f, "Template not found: {}", name),
            ScxVoidError::UnsupportedProjectType(index) => {
                write!(f, "Unsupported project type index: {}", index)
            }
            ScxVoidError::ClaudeRuleFileExists(path) => {
                write!(f, "Claude rule file already exists: {:?}. Use --force to overwrite.", path)
            }
            ScxVoidError::InvalidTemplateType(template) => {
                write!(f, "Invalid template type: {}. Available: basic, advanced", template)
            }
            ScxVoidError::AudioFileNotFound(path) => {
                write!(f, "Audio file not found: {}", path)
            }
            ScxVoidError::UnsupportedAudioFormat(format) => {
                write!(f, "Unsupported audio format: {}. Supported: M4A, AAC, MP4", format)
            }
            ScxVoidError::AudioDecodingError(msg) => {
                write!(f, "Audio decoding error: {}", msg)
            }
            ScxVoidError::WhisperModelNotFound(path) => {
                write!(f, "Whisper model not found: {}. Use 'scx-void audio download-model' to download a model.", path)
            }
            ScxVoidError::WhisperLoadError(msg) => {
                write!(f, "Failed to load Whisper model: {}", msg)
            }
            ScxVoidError::TranscriptionError(msg) => {
                write!(f, "Transcription failed: {}", msg)
            }
            ScxVoidError::ModelDownloadError(msg) => {
                write!(f, "Model download failed: {}", msg)
            }
            ScxVoidError::NetworkError(msg) => {
                write!(f, "Network error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ScxVoidError {}
