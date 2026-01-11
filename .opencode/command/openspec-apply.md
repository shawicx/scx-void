---
description: 实施已批准的OpenSpec变更并保持任务同步。
---
用户请求实施以下变更提案。查找变更提案并按照以下说明操作。如果有疑问或不明确，请向用户寻求澄清。
<UserRequest>
  $ARGUMENTS
</UserRequest>
<!-- OPENSPEC:START -->
**准则**
- 首先采用直接、最小化的实现，仅在被要求或明显需要时才增加复杂性。
- 将变更范围严格控制在所请求的结果内。
- 如需额外的OpenSpec约定或澄清，请参考 `openspec/AGENTS.md`（位于 `openspec/` 目录内——如果看不到，请运行 `ls openspec` 或 `openspec update`）。

**步骤**
将这些步骤作为待办事项跟踪，并逐一完成。
1. 阅读 `changes/<id>/proposal.md`、`design.md`（如果存在）和 `tasks.md` 以确认范围和验收标准。
2. 按顺序处理任务，保持编辑最小化且专注于所请求的变更。
3. 更新状态前确认完成情况——确保 `tasks.md` 中的每一项都已完成。
4. 所有工作完成后更新检查列表，使每个任务都标记为 `- [x]` 并反映实际情况。
5. 需要额外上下文时，请参考 `openspec list` 或 `openspec show <item>`。

**参考**
- 实施过程中如需从提案中获取额外上下文，请使用 `openspec show <id> --json --deltas-only`。
<!-- OPENSPEC:END -->