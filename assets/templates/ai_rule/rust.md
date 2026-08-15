## Rust 项目规则

### 构建工具与运行时

1. **必须使用 `cargo` 作为构建与包管理工具**，不得擅自引入其他构建系统（bazel / cmake 等，除非项目已使用）
2. **Rust edition 必须与 `Cargo.toml` 中 `[package].edition` 字段一致**（通常为 2021 或 2024），不得擅自切换
3. 工具链版本（stable / nightly）遵循 `rust-toolchain.toml` 或项目现状，不得擅自更改

### 项目结构约定

遵循 Cargo 标准结构，不得擅自调整一级目录：

- `src/main.rs` — 二进制入口（应用类型）
- `src/lib.rs` — 库 crate 入口（库类型）
- `src/bin/` — 额外可执行文件
- `src/<module>/` 或 `src/<module>.rs` — 模块（遵循项目现有模块划分）
- `tests/` — 集成测试
- `benches/` — 性能基准测试（criterion 等）
- `examples/` — 示例代码
- `Cargo.toml` — 依赖与清单
- `Cargo.lock` — 锁文件（二进制必须提交，库遵循项目现状）

### 构建与测试命令

- `cargo build` — 编译
- `cargo run` — 运行
- `cargo test` — 运行所有测试
- `cargo test --test <name>` — 运行指定集成测试
- `cargo fmt` — 格式化
- `cargo clippy` — 静态检查（必须零警告通过）
- `cargo check` — 快速类型检查
- `cargo bench` — 运行基准测试

### 依赖与配置规则

1. **禁止降级依赖版本**，遵循 cargo semver；重大升级需经用户批准
2. 新增依赖优先检查是否与现有 `features` 冲突，避免重复引入功能重叠的 crate
3. `Cargo.toml` 修改采取合并策略，不得重写整个文件
4. `unsafe` 代码必须有必要性说明注释，非必要不使用 `unsafe`
5. 公开 API（`pub fn` / `pub struct` / `pub enum` 等）必须有 `///` 文档注释，遵循 rustdoc 规范
6. 错误处理遵循项目现状（`anyhow` / `thiserror` / 自定义 Error 类型），不得擅自替换方案

### 禁止事项

- 不得擅自切换 Rust edition（2021 ↔ 2024 需用户明确批准）
- 不得在库项目中提交 `Cargo.lock`（或在二进制项目中删除它），除非遵循项目现状
- 不得引入 `unsafe` 块而无必要性注释
- 不得擅自替换错误处理方案（`anyhow` ↔ `thiserror` ↔ 自定义）
- 不得绕过 `cargo fmt` / `cargo clippy` 的既有规则
- 不得重写 `Cargo.toml` 整个文件
