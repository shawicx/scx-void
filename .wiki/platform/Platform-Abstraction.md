# 平台抽象层

**关联代码**：`src/platform/`

平台抽象层使用 **Trait + 编译时选择** 模式，确保平台无关的业务逻辑不直接依赖系统命令。

## SystemOps Trait

**关联代码**：`src/platform/mod.rs`

```rust
pub trait SystemOps {
    fn shutdown_in(&self, seconds: u64) -> duct::Expression;
}
```

## 平台实现

### macOS

**关联代码**：`src/platform/macos.rs`

- `MacosPlatform` 实现 `SystemOps`
- 关机时将秒数转换为分钟（向上取整）
- 使用 `duct` 执行 `sudo shutdown -h +<minutes>`

### Windows

**关联代码**：`src/platform/windows.rs`

- `WindowsPlatform` 实现 `SystemOps`
- 直接使用秒数
- 使用 `duct` 执行 `shutdown /s /t <seconds>`

## 编译时选择

通过 `#[cfg(target_os = "...")]` 在编译时确定使用哪个平台实现：

```rust
#[cfg(target_os = "macos")]
pub fn get_platform() -> impl SystemOps {
    MacosPlatform {}
}

#[cfg(target_os = "windows")]
pub fn get_platform() -> impl SystemOps {
    WindowsPlatform {}
}
```

## 扩展新平台

添加新平台支持的步骤：

1. 在 `src/platform/` 下创建平台文件（如 `linux.rs`）
2. 实现 `SystemOps` trait
3. 在 `mod.rs` 中添加 `#[cfg(target_os = "...")]` 条件导出

---

## 相关文档

- [架构概览](../Overview/Architecture.md)
- [系统服务](../services/system/System-Service.md)
- [错误处理](../Overview/Error-Handling.md)

---

> 最后更新：2026-04-25
