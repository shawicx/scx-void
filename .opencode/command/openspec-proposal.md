---
description: 构建新的OpenSpec变更并进行严格验证。
---
用户请求了以下变更提案。使用openspec说明创建他们的变更提案。
<UserRequest>
  $ARGUMENTS
</UserRequest>
<!-- OPENSPEC:START -->
**准则**
- 首先采用直接、最小化的实现，仅在被要求或明显需要时才增加复杂性。
- 将变更范围严格控制在所请求的结果内。
- 如需额外的OpenSpec约定或澄清，请参考 `openspec/AGENTS.md`（位于 `openspec/` 目录内——如果看不到，请运行 `ls openspec` 或 `openspec update`）。
- 识别任何模糊或不明确的细节，并在编辑文件之前提出必要的后续问题。
- 在提案阶段不要编写任何代码。只创建设计文档（proposal.md、tasks.md、design.md和规范增量）。实现在批准后的应用阶段进行。

**步骤**
1. 查阅 `openspec/project.md`，运行 `openspec list` 和 `openspec list --specs`，并检查相关代码或文档（例如，通过 `rg`/`ls`）以使提案基于当前行为；注意需要澄清的任何差距。
2. 选择一个独特的动词引导的 `change-id` 并在 `openspec/changes/<id>/` 下构建 `proposal.md`、`tasks.md` 和 `design.md`（如需要）。
3. 将变更映射到具体功能或需求，将多范围的工作分解为具有清晰关系和顺序的不同规范增量。
4. 当解决方案跨越多个系统、引入新模式或在提交规范前需要权衡讨论时，在 `design.md` 中记录架构推理。
5. 在 `changes/<id>/specs/<capability>/spec.md` 中起草规范增量（每个功能一个文件夹），使用 `## ADDED|MODIFIED|REMOVED Requirements` 并为每个需求至少提供一个 `#### Scenario:`，并在相关时交叉引用相关功能。
6. 起草 `tasks.md` 作为小的、可验证的工作项目的有序列表，这些项目能提供用户可见的进展，包括验证（测试、工具），并突出依赖关系或可并行的工作。
7. 使用 `openspec validate <id> --strict` 进行验证，并在共享提案之前解决每个问题。

**参考**
- 验证失败时使用 `openspec show <id> --json --deltas-only` 或 `openspec show <spec> --type spec` 检查详细信息。
- 编写新需求之前使用 `rg -n "Requirement:|Scenario:" openspec/specs` 搜索现有需求。
- 使用 `rg <keyword>`、`ls` 或直接文件读取探索代码库，以便提案与当前实现现实保持一致。
<!-- OPENSPEC:END -->