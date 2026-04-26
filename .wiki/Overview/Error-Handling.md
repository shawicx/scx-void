# 错误处理

项目使用集中化错误类型体系，所有错误变体统一定义在 `src/errors.rs`，通过 `thiserror` crate 自动实现 `Display` 和 `Error` trait。

**关联代码**：`src/errors.rs`

## 错误类型一览

### 通用错误

| 变体 | 说明 |
|------|------|
| `GeneralError(String)` | 通用错误 |
| `FileSystemError(String)` | 文件系统操作失败 |
| `NetworkError(String)` | 网络请求失败 |

### 项目相关错误

| 变体 | 说明 |
|------|------|
| `InvalidProjectName` | 无效的项目名称 |
| `ProjectAlreadyExists` | 项目目录已存在 |
| `TemplateNotFound` | 模板未找到 |
| `UnsupportedProjectType` | 不支持的项目类型 |
| `AiRuleFileExists` | AGENTS.md 已存在（未指定 `--force`） |
| `InvalidTemplateType` | 无效的模板类型 |
| `InvalidGitHubUrl` | 无效的 GitHub URL 格式 |
| `ArchiveExtractError` | zip 归档解压失败 |
| `TemplateDownloadFailed` | 模板下载失败（HTTP 错误等） |
| `GitTemplateNotFound` | 模板 ID 不存在 |
| `GitBranchNotFound` | Git 分支名无效 |

### 音频相关错误

| 变体 | 说明 |
|------|------|
| `AudioFileNotFound` | 音频文件不存在 |
| `UnsupportedAudioFormat` | 不支持的音频格式 |
| `AudioDecodingError` | 音频解码失败 |
| `WhisperModelNotFound` | Whisper 模型未找到 |
| `WhisperLoadError` | Whisper 模型加载失败 |
| `TranscriptionError` | 转写过程失败 |
| `ModelDownloadError` | 模型下载失败 |

### 自动错误转换

`errors.rs` 中为外部 crate 错误实现了 `From` trait：

```rust
impl From<reqwest::Error> for ScxVoidError     // HTTP 错误 → NetworkError
impl From<zip::result::ZipError> for ScxVoidError // zip 错误 → ArchiveExtractError
```

## 使用规范

### 错误传播

使用 `?` 操作符进行错误传播，所有可能失败的操作返回 `Result<T, ScxVoidError>`：

```rust
fn create_project(name: &str, template: &GitTemplate) -> Result<(), ScxVoidError> {
    let (temp_dir, source_dir) = downloader::download_template_to_temp(template, None).await?;
    downloader::extract_template_files(&source_dir, name)?;
    Ok(())
}
```

### 外部错误转换

使用 `.map_err()` 将外部 crate 的错误转换为 `ScxVoidError`：

```rust
let file = fs::read_to_string(path).map_err(|e| {
    ScxVoidError::FileSystemError(format!("读取文件失败: {}", e))
})?;
```

### 禁止事项

- **禁止使用 `unwrap()`** — 所有可能失败的操作必须使用 `?` 或显式错误处理
- **禁止在 `errors.rs` 之外定义错误类型** — 保持错误类型集中管理
- **禁止静默忽略错误** — 所有错误必须传播或记录

## 相关文档

- [架构概览](Architecture.md)
- [开发指南](Development-Guide.md)

---

> 最后更新：2026-04-26
