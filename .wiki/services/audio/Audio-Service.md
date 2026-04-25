# 音频服务

音频服务提供基于 OpenAI Whisper 模型的语音转文字功能，包括音频解码、模型管理和转写三个核心模块。

> **Feature Gate**：音频功能需要启用 `audio` feature 编译。
> `cargo build --features audio`

## 模块总览

```
src/services/audio/
├── mod.rs              # 模块导出
├── audio_service.rs    # 转写服务主逻辑
├── decoder.rs          # 音频解码器（symphonia）
├── model.rs            # Whisper 模型管理
└── whisper.rs          # Whisper 转写引擎集成
```

---

## AudioService — 转写服务主逻辑

**关联代码**：`src/services/audio/audio_service.rs`

`AudioService` 是音频子系统的门面，组合了 `AudioDecoder`、`ModelManager` 和 `WhisperTranscriber`。

### 关键方法

| 方法 | 说明 |
|------|------|
| `new()` | 初始化服务实例 |
| `transcribe_file(path, model, lang)` | 基础转写，返回纯文本 |
| `transcribe_with_timestamps(path, model, lang, opts)` | 带时间戳的 SRT 格式转写 |
| `transcribe_file_with_advanced_params(...)` | 完整参数的高级转写 |
| `download_model(name)` | 下载 Whisper 模型 |
| `list_models()` | 列出模型及其状态 |

### 转写流程

```
用户提供音频文件 + 模型 + 语言
            │
            ▼
    AudioDecoder::decode_to_pcm()
    解码为 16kHz 单声道 PCM
            │
            ▼
    ModelManager::get_model_path()
    定位模型文件
            │
            ▼
    WhisperTranscriber::transcribe()
    执行语音转文字
            │
      ┌─────┴─────┐
      ▼           ▼
  纯文本      SRT 时间戳
```

### 时间过滤

支持跳过指定秒数和截取指定时长：
- `skip_seconds` — 跳过前 N 秒
- `end_time` — 截止时间点（秒）

---

## AudioDecoder — 音频解码器

**关联代码**：`src/services/audio/decoder.rs`

负责将音频文件解码为 Whisper 可用的 PCM 格式。

### 关键方法

| 方法 | 说明 |
|------|------|
| `decode_to_pcm(path)` | 将音频文件转为 16kHz 单声道 PCM 数据 |
| `downmix_to_mono(samples)` | 立体声降混为单声道 |
| `resample(samples, from, to)` | 重采样到目标采样率 |

### 支持的格式

通过 `symphonia` crate（启用 `aac` + `isomp4` features）支持：

| 格式 | 扩展名 |
|------|--------|
| M4A | `.m4a` |
| AAC | `.aac` |
| MP4 | `.mp4` |

### 处理管线

```
原始音频文件
    │
    ▼
symphonia 解码
    │
    ▼
降混为单声道（如为立体声）
    │
    ▼
重采样到 16kHz（如采样率不同）
    │
    ▼
16-bit PCM 数据
```

---

## ModelManager — 模型管理

**关联代码**：`src/services/audio/model.rs`

管理 Whisper 模型的下载、存储和查询。

### 关键结构

| 结构 | 说明 |
|------|------|
| `ModelManager` | 模型管理器，持有模型存储目录 |
| `ModelInfo` | 模型元数据（名称、大小、状态） |

### 关键方法

| 方法 | 说明 |
|------|------|
| `new()` | 初始化，使用 `~/.scx-void/models` 作为存储目录 |
| `get_available_models()` | 列出所有可用模型（含下载状态） |
| `download_model(name)` | 下载模型，带进度条 |
| `get_model_path(name_or_path)` | 智能查找：先按名称，再按路径 |
| `list_downloaded_models()` | 仅列出已下载的模型 |
| `validate_model(path)` | 验证模型文件有效性 |

### 模型存储

```
~/.scx-void/models/
├── ggml-base.bin
├── ggml-small.bin
├── ggml-medium.bin
└── ...
```

### 下载流程

```
请求下载模型
    │
    ▼
检查本地缓存 → 已存在 → 跳过
    │
    ▼ 未下载
构建下载 URL
    │
    ▼
reqwest 流式下载 + indicatif 进度条
    │
    ▼
写入临时文件 → 验证 → 移至最终路径
```

---

## WhisperTranscriber — Whisper 引擎集成

**关联代码**：`src/services/audio/whisper.rs`

封装 `whisper-rs` crate，提供 Rust 友好的转写接口。

### 关键结构

| 结构 | 说明 |
|------|------|
| `WhisperTranscriber` | Whisper 上下文封装 |
| `TranscriptionSegment` | 转写片段（起止时间 + 文本） |

### 关键方法

| 方法 | 说明 |
|------|------|
| `new(model_path)` | 加载 Whisper 模型创建上下文 |
| `transcribe(pcm, lang)` | 基础转写 |
| `transcribe_with_params(pcm, lang, params)` | 高级参数转写 |
| `transcribe_to_text(pcm, lang)` | 仅提取文本 |
| `validate_whisper_model(path)` | 验证模型文件 |

### 高级参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `temperature` | 0.0-1.0 | 采样温度，越低越确定 |
| `beam_size` | usize | Beam search 宽度，影响准确率 |
| `no_speech_threshold` | f32 | 静音片段过滤阈值 |

### TranscriptionSegment

```rust
struct TranscriptionSegment {
    start_ms: u64,   // 起始毫秒
    end_ms: u64,     // 结束毫秒
    text: String,    // 转写文本
}
```

- `duration_ms()` — 计算片段时长

---

## 相关文档

- [架构概览](../../Overview/Architecture.md)
- [CLI 命令参考 — audio](../../cli/CLI-Reference.md#audio--音频转写可选功能)
- [错误处理](../../Overview/Error-Handling.md)

---

> 最后更新：2026-04-25
