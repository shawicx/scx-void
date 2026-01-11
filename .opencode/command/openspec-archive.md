---
description: 归档已部署的OpenSpec变更并更新规范。
---
<ChangeId>
  $ARGUMENTS
</ChangeId>
<!-- OPENSPEC:START -->
**准则**
- 首先采用直接、最小化的实现，仅在被要求或明显需要时才增加复杂性。
- 将变更范围严格控制在所请求的结果内。
- 如需额外的OpenSpec约定或澄清，请参考 `openspec/AGENTS.md`（位于 `openspec/` 目录内——如果看不到，请运行 `ls openspec` 或 `openspec update`）。

**步骤**
1. 确定要归档的变更ID：
   - 如果此提示已包含特定变更ID（例如在由斜杠命令参数填充的 `<ChangeId>` 块内），请在去除空白字符后使用该值。
   - 如果对话中模糊引用了变更（例如按标题或摘要），请运行 `openspec list` 以显示可能的ID，分享相关候选者，并确认用户想要的是哪一个。
   - 否则，请查看对话，运行 `openspec list`，并询问用户要归档哪个变更；在继续之前等待确认的变更ID。
   - 如果仍然无法确定单个变更ID，请停止并告诉用户目前无法归档任何内容。
2. 通过运行 `openspec list`（或 `openspec show <id>`）验证变更ID，如果变更缺失、已经归档或以其他方式未准备好归档，则停止。
3. 运行 `openspec archive <id> --yes`，以便CLI在没有提示的情况下移动变更并应用规范更新（仅在纯工具工作中使用 `--skip-specs`）。
4. 查看命令输出以确认目标规范已更新且变更已进入 `changes/archive/`。
5. 使用 `openspec validate --strict` 进行验证，如有异常，请使用 `openspec show <id>` 检查。

**参考**
- 使用 `openspec list` 在归档前确认变更ID。
- 使用 `openspec list --specs` 检查刷新的规范，并在移交前解决任何验证问题。
<!-- OPENSPEC:END -->