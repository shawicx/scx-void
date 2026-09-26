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
    /// 未知的技术栈类型（ai-rule 命令的 -t 参数）
    UnknownStackType(String),
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
    /// 不支持的压缩格式（非 JPEG/PNG/WebP，或扩展名与内容不符）
    UnsupportedCompressFormat(String),
    /// 图片压缩失败
    CompressionFailed {
        source: String,
        reason: String,
    },
    /// 系统未安装压缩工具
    CompressorNotFound {
        tool: String,
        hint: String,
    },
    /// release：无法识别目标项目类型
    ReleaseProjectNotDetected(String),
    /// release：版本号格式无效
    InvalidVersion(String),
    /// release：package.json 缺少 version 字段
    VersionFieldMissing(std::path::PathBuf),
    /// release：git tag 已存在
    TagAlreadyExists(String),
    /// release：当前目录不是 git 仓库
    NotAGitRepository,
    /// release：工作区存在未提交改动
    DirtyWorktree(String),
    /// release：package.json 标记为 private，无法发布 npm
    PrivatePackage,
    /// release：git 仓库未配置远程
    GitRemoteMissing,
    /// release：未安装发布所需工具
    ReleaseToolNotFound {
        tool: String,
        hint: String,
    },
    /// release：git 命令执行失败
    GitCommandFailed {
        command: String,
        reason: String,
    },
    /// release：发布步骤执行失败（含已完成步骤与手动补做提示）
    ReleaseStepFailed {
        step: String,
        completed: Vec<String>,
        reason: String,
    },
    /// release：package.json / tauri.conf.json / Cargo.toml 编辑失败
    ManifestEditError {
        path: String,
        reason: String,
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
            ScxVoidError::UnknownStackType(id) => {
                write!(
                    f,
                    "未知的技术栈类型 '{}'。可用类型：vue3, react, nextjs, node-cli, nestjs, tauri, java",
                    id
                )
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
            ScxVoidError::ImageConversionFailed {
                source,
                target,
                reason,
            } => {
                write!(f, "从 {} 转换到 {} 失败: {}", source, target, reason)
            }
            ScxVoidError::ConverterNotFound { tool, hint } => {
                write!(f, "未找到转换工具 '{}': {}", tool, hint)
            }
            ScxVoidError::UnsupportedCompressFormat(msg) => {
                write!(f, "不支持的压缩格式: {}", msg)
            }
            ScxVoidError::CompressionFailed { source, reason } => {
                write!(f, "压缩 '{}' 失败: {}", source, reason)
            }
            ScxVoidError::CompressorNotFound { tool, hint } => {
                write!(f, "未找到压缩工具 '{}': {}", tool, hint)
            }
            ScxVoidError::ReleaseProjectNotDetected(msg) => {
                write!(f, "无法识别项目类型: {}", msg)
            }
            ScxVoidError::InvalidVersion(v) => write!(f, "无效的版本号: {}", v),
            ScxVoidError::VersionFieldMissing(path) => {
                write!(f, "package.json 缺少 version 字段: {:?}", path)
            }
            ScxVoidError::TagAlreadyExists(tag) => write!(f, "git tag '{}' 已存在", tag),
            ScxVoidError::NotAGitRepository => write!(f, "当前目录不是 git 仓库"),
            ScxVoidError::DirtyWorktree(status) => {
                write!(f, "工作区存在未提交改动，请先提交或暂存:\n{}", status)
            }
            ScxVoidError::PrivatePackage => {
                write!(f, "package.json 中 private 为 true，无法发布到 npm")
            }
            ScxVoidError::GitRemoteMissing => {
                write!(f, "git 仓库未配置远程 (remote)，无法推送")
            }
            ScxVoidError::ReleaseToolNotFound { tool, hint } => {
                write!(f, "未找到发布所需工具 '{}': {}", tool, hint)
            }
            ScxVoidError::GitCommandFailed { command, reason } => {
                write!(f, "命令 '{}' 执行失败: {}", command, reason)
            }
            ScxVoidError::ReleaseStepFailed {
                step,
                completed,
                reason,
            } => {
                if completed.is_empty() {
                    write!(f, "发布步骤 '{}' 失败: {}", step, reason)
                } else {
                    write!(
                        f,
                        "发布步骤 '{}' 失败: {}\n已完成步骤: {}\n可手动补做后续步骤",
                        step,
                        reason,
                        completed.join(" -> ")
                    )
                }
            }
            ScxVoidError::ManifestEditError { path, reason } => {
                write!(f, "文件 '{}' 编辑失败: {}", path, reason)
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

impl From<serde_json::Error> for ScxVoidError {
    fn from(err: serde_json::Error) -> Self {
        ScxVoidError::ManifestEditError {
            path: "JSON 解析".to_string(),
            reason: err.to_string(),
        }
    }
}
