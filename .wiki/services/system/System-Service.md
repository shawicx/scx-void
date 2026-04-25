# 系统服务

`ShutdownService` 提供跨平台的系统关机能力。

**关联代码**：`src/services/system/shutdown.rs`

## 关键方法

| 方法 | 说明 |
|------|------|
| `shutdown_in(seconds)` | 在指定秒数后关机 |

## 平台命令映射

| 平台 | 命令 | 时间单位 |
|------|------|----------|
| macOS | `sudo shutdown -h +<分钟>` | 分钟（自动转换） |
| Windows | `shutdown /s /t <秒>` | 秒 |
| Linux | `shutdown -h +<秒>` | 秒 |

> macOS 会将秒数向上取整为分钟。

---

## 相关文档

- [架构概览](../../Overview/Architecture.md)
- [CLI 命令参考 — system](../../cli/CLI-Reference.md#system--系统操作)
- [错误处理](../../Overview/Error-Handling.md)
- [平台抽象层](../../platform/Platform-Abstraction.md)

---

> 最后更新：2026-04-25
