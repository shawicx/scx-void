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

## GitHub URL 工具

**关联代码**：`src/utils/git.rs`

| 函数 | 说明 |
|------|------|
| `parse_github_url(url)` | 解析 GitHub URL 为 `(owner, repo)` 元组 |
| `build_archive_url(owner, repo, branch)` | 构建 codeload zip 下载 URL |
| `validate_git_url(url)` | 验证 GitHub URL 格式有效性 |

`parse_github_url` 支持的 URL 格式：
- HTTPS：`https://github.com/owner/repo`
- 短格式：`owner/repo`
- 带 `.git` 后缀：`owner/repo.git`

`build_archive_url` 生成的下载 URL 格式：
```
https://codeload.github.com/{owner}/{repo}/zip/refs/heads/{branch}
```

## Shell 工具

**关联代码**：`src/utils/shell.rs`

预留的 Shell 工具模块，当前为最小实现。

---

## 相关文档

- [架构概览](../Overview/Architecture.md)
- [错误处理](../Overview/Error-Handling.md)

---

> 最后更新：2026-04-26
