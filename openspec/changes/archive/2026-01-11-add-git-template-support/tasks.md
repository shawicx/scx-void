## 1. 基础设施准备

- [x] 1.1 创建 `src/services/project/git/` 模块目录结构
- [x] 1.2 实现 `src/services/project/git/types.rs` - 定义所有数据结构（ProjectType、GitTemplate、CloneOptions、TemplateSource）
- [x] 1.3 实现 `src/services/project/git/registry.rs` - 模板注册表和预定义模板配置
- [x] 1.4 实现 `src/services/project/git/validator.rs` - URL 和参数验证逻辑
- [x] 1.5 实现 `src/utils/git.rs` - Git 命令工具函数（is_git_installed、validate_git_url 等）
- [x] 1.6 修改 `src/errors.rs` - 添加 Git 相关错误类型（GitCloneError、InvalidGitUrl、GitNotInstalled 等）

## 2. 核心功能实现

- [x] 2.1 实现 `src/services/project/git/clone.rs` - Git 克隆功能（clone_repository、clone_sparse_checkout）
- [x] 2.2 实现 `src/services/project/git/downloader.rs` - 模板下载器（download_template_to_temp、extract_template_files）
- [x] 2.3 修改 `src/services/project/generator.rs` - 集成 Git 模板生成逻辑
- [x] 2.4 修改 `src/services/project/project_service.rs` - 添加 create_project_from_git 和统一入口函数
- [x] 2.5 实现 `src/services/project/git/mod.rs` - 模块入口，导出公共接口

## 3. 单元测试

- [x] 3.1 编写 `src/utils/git.rs` 单元测试 - Git 工具函数测试
- [x] 3.2 编写 `src/services/project/git/validator.rs` 单元测试 - 验证逻辑测试
- [x] 3.3 编写 `src/services/project/git/registry.rs` 单元测试 - 模板注册表测试
- [x] 3.4 编写 `src/services/project/git/clone.rs` 单元测试 - Git 克隆功能测试（使用模拟或测试仓库）

## 4. CLI 集成

- [x] 4.1 修改 `src/cli/project.rs` - 添加 Git 源相关 CLI 参数（--source、--template、--url、--branch、--template_path）
- [x] 4.2 实现交互式模板源选择流程（select_template_source）
- [x] 4.3 实现 Git 模板选择交互（select_git_template）
- [x] 4.4 实现自定义模板输入流程（input_custom_template）
- [x] 4.5 实现命令式参数解析和路由逻辑
- [x] 4.6 添加用户友好的提示信息和错误反馈

## 5. 集成测试

- [ ] 5.1 编写 `tests/git_template_test.rs` - 集成测试（使用预定义模板的完整项目创建流程）
- [ ] 5.2 编写 `tests/git_template_custom_url_test.rs` - 集成测试（使用自定义 URL 的完整项目创建流程）
- [ ] 5.3 编写 `tests/git_template_branch_test.rs` - 集成测试（指定分支的完整项目创建流程）
- [ ] 5.4 编写 `tests/git_template_path_test.rs` - 集成测试（指定模板路径的完整项目创建流程）
- [ ] 5.5 编写 `tests/mixed_mode_test.rs` - 集成测试（混合模式：部分参数 + 交互式输入）
- [ ] 5.6 编写 `tests/backward_compatibility_test.rs` - 集成测试（验证本地模板功能不受影响）

## 6. 错误处理和优化

- [x] 6.1 优化错误处理和错误消息
- [ ] 6.2 添加下载进度提示（克隆大仓库时）
- [x] 6.3 实现 TempDir 自动清理验证
- [x] 6.4 添加 Git 未安装的检测和友好提示
- [ ] 6.5 添加网络超时处理

## 7. 文档和代码质量

- [ ] 7.1 更新项目 README - 添加 Git 模板功能说明和使用示例
- [ ] 7.2 编写使用文档 - 交互式和命令式使用示例
- [ ] 7.3 为所有公共函数添加文档注释（`///`）
- [x] 7.4 验证所有公共结构体实现了 `Debug` 和 `Clone` trait
- [x] 7.5 运行 `cargo clippy` 检查并修复所有警告
- [x] 7.6 运行 `cargo fmt` 确保代码格式符合标准
- [x] 7.7 运行 `cargo test --all` 确保所有测试通过

## 8. 依赖管理

- [x] 8.1 更新 `Cargo.toml` - 添加 `tempfile = "3.x"` 依赖
- [x] 8.2 更新 `Cargo.toml` - 添加 `url = "2.x"` 依赖
- [x] 8.3 验证所有依赖已正确配置

## 9. 最终验证

- [ ] 9.1 验证交互式模式 - 完整测试所有交互式流程
- [ ] 9.2 验证命令式模式 - 完整测试所有命令式参数组合
- [ ] 9.3 验证混合模式 - 测试部分参数 + 交互式输入
- [ ] 9.4 验证向后兼容性 - 确保本地模板功能正常工作
- [ ] 9.5 验证错误处理 - 测试各种错误场景
- [ ] 9.6 性能测试 - 验证浅克隆优化效果
- [ ] 9.7 跨平台测试 - 在 macOS、Linux、Windows 上测试（如果可能）

## 依赖关系说明

- **阶段 1 → 阶段 2**：基础设施必须先完成，才能实现核心功能
- **阶段 2 → 阶段 3**：核心功能实现后才能编写单元测试
- **阶段 2 → 阶段 4**：核心功能实现后才能集成到 CLI
- **阶段 4 → 阶段 5**：CLI 集成完成后才能编写集成测试
- **阶段 1-5 → 阶段 6**：功能实现和测试完成后才能进行优化
- **所有阶段 → 阶段 7**：所有功能完成后才能编写文档和进行代码质量检查
- **阶段 8**：依赖管理可以在任何时候进行，但建议在阶段 1 开始前完成
- **所有阶段 → 阶段 9**：最终验证必须在所有其他任务完成后进行

## 并行任务

以下任务可以并行执行（在满足依赖关系的前提下）：
- 1.2、1.3、1.4、1.5 可以并行开发（数据结构独立）
- 3.1、3.2、3.3 可以并行编写（单元测试独立）
- 5.1、5.2、5.3、5.4 可以并行编写（集成测试独立）
