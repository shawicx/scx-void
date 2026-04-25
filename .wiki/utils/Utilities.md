# 工具函数层

**关联代码**：`src/utils/`

## 文件系统工具

**关联代码**：`src/utils/fs.rs`

| 函数 | 说明 |
|------|------|
| `create_dir(path)` | 创建目录（含父目录） |
| `write_file(path, content)` | 写入文件内容 |
| `copy_dir_all(src, dst)` | 递归复制目录 |
| `copy_file(src, dst)` | 复制单个文件 |

## Git 工具

**关联代码**：`src/utils/git.rs`

| 函数 | 说明 |
|------|------|
| `is_git_installed()` | 检测 Git 是否安装 |
| `validate_git_url(url)` | 验证 Git URL 格式 |
| `get_default_branch(url)` | 获取仓库默认分支 |
| `branch_exists(url, branch)` | 检查分支是否存在 |
| `list_branches(url)` | 列出所有分支 |

支持的 URL 格式：
- HTTPS：`https://github.com/user/repo.git`
- SSH：`git@github.com:user/repo.git`
- 短格式：`user/repo`

## Shell 工具

**关联代码**：`src/utils/shell.rs`

预留的 Shell 工具模块，当前为最小实现。

---

## 相关文档

- [架构概览](../Overview/Architecture.md)
- [错误处理](../Overview/Error-Handling.md)

---

> 最后更新：2026-04-25
