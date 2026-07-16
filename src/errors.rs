#[derive(Debug)]
#[allow(dead_code)]
pub enum ScxVoidError {
    GeneralError(String),
    InvalidProjectName(String),
    ProjectAlreadyExists(String),
    FileSystemError(String),
    TemplateNotFound(String),
    UnsupportedProjectType(usize),
    AiRuleFileExists(std::path::PathBuf),
    InvalidTemplateType(String),
    AudioFileNotFound(String),
    UnsupportedAudioFormat(String),
    AudioDecodingError(String),
    WhisperModelNotFound(String),
    WhisperLoadError(String),
    TranscriptionError(String),
    ModelDownloadError(String),
    NetworkError(String),
    /// 归档文件解压失败
    ArchiveExtractError(String),
    /// 无效的 GitHub URL
    InvalidGitHubUrl(String),
    /// Git 分支不存在
    GitBranchNotFound(String),
    /// 模板下载失败
    TemplateDownloadFailed(String),
    /// Git 模板 ID 不存在
    GitTemplateNotFound(String),
    /// 安装失败
    InstallationFailed {
        component: String,
        reason: String,
    },
    /// Shell 配置写入失败
    ShellConfigError {
        path: String,
        reason: String,
    },
    /// 不支持的图像格式（无法识别，或扩展名与内容不符）
    UnsupportedImageFormat(String),
    /// 图像转换失败
    ImageConversionFailed {
        source: String,
        target: String,
        reason: String,
    },
    /// 系统未安装所需的转换工具
    ConverterNotFound {
        tool: String,
        hint: String,
    },
}

impl std::fmt::Display for ScxVoidError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ScxVoidError::GeneralError(msg) => write!(f, "错误: {}", msg),
            ScxVoidError::InvalidProjectName(msg) => write!(f, "无效的项目名称: {}", msg),
            ScxVoidError::ProjectAlreadyExists(name) => {
                write!(f, "项目 '{}' 已存在", name)
            }
            ScxVoidError::FileSystemError(msg) => write!(f, "文件系统错误: {}", msg),
            ScxVoidError::TemplateNotFound(name) => {
                write!(f, "模板 '{}' 不存在", name)
            }
            ScxVoidError::UnsupportedProjectType(index) => {
                write!(f, "不支持的项目类型索引: {}", index)
            }
            ScxVoidError::AiRuleFileExists(path) => {
                write!(f, "AI 规则文件已存在: {:?}", path)
            }
            ScxVoidError::InvalidTemplateType(msg) => {
                write!(f, "无效的模板类型: {}", msg)
            }
            ScxVoidError::AudioFileNotFound(msg) => {
                write!(f, "音频文件未找到: {}", msg)
            }
            ScxVoidError::UnsupportedAudioFormat(msg) => {
                write!(f, "不支持的音频格式: {}", msg)
            }
            ScxVoidError::AudioDecodingError(msg) => {
                write!(f, "音频解码错误: {}", msg)
            }
            ScxVoidError::WhisperModelNotFound(msg) => {
                write!(f, "Whisper 模型未找到: {}", msg)
            }
            ScxVoidError::WhisperLoadError(msg) => {
                write!(f, "Whisper 模型加载失败: {}", msg)
            }
            ScxVoidError::TranscriptionError(msg) => {
                write!(f, "转录错误: {}", msg)
            }
            ScxVoidError::ModelDownloadError(msg) => {
                write!(f, "模型下载错误: {}", msg)
            }
            ScxVoidError::NetworkError(msg) => {
                write!(f, "网络错误: {}", msg)
            }
            ScxVoidError::ArchiveExtractError(msg) => {
                write!(f, "归档文件解压失败: {}", msg)
            }
            ScxVoidError::InvalidGitHubUrl(msg) => {
                write!(f, "无效的 GitHub URL: {}", msg)
            }
            ScxVoidError::GitBranchNotFound(branch) => {
                write!(f, "分支 '{}' 不存在或无效", branch)
            }
            ScxVoidError::TemplateDownloadFailed(msg) => {
                write!(f, "模板下载失败: {}", msg)
            }
            ScxVoidError::GitTemplateNotFound(id) => {
                write!(f, "模板 ID '{}' 不存在", id)
            }
            ScxVoidError::InstallationFailed { component, reason } => {
                write!(f, "安装 '{}' 失败: {}", component, reason)
            }
            ScxVoidError::ShellConfigError { path, reason } => {
                write!(f, "Shell 配置文件 '{}' 写入失败: {}", path, reason)
            }
            ScxVoidError::UnsupportedImageFormat(msg) => {
                write!(f, "不支持的图像格式: {}", msg)
            }
            ScxVoidError::ImageConversionFailed { source, target, reason } => {
                write!(f, "从 {} 转换到 {} 失败: {}", source, target, reason)
            }
            ScxVoidError::ConverterNotFound { tool, hint } => {
                write!(f, "未找到转换工具 '{}': {}", tool, hint)
            }
        }
    }
}

impl std::error::Error for ScxVoidError {}

impl From<reqwest::Error> for ScxVoidError {
    fn from(err: reqwest::Error) -> Self {
        ScxVoidError::NetworkError(err.to_string())
    }
}

impl From<zip::result::ZipError> for ScxVoidError {
    fn from(err: zip::result::ZipError) -> Self {
        ScxVoidError::ArchiveExtractError(err.to_string())
    }
}
