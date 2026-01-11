# OpenSpec 说明

供AI编码助手使用OpenSpec进行规范驱动开发的说明。

## 快速清单（TL;DR）

- 搜索现有工作：`openspec spec list --long`，`openspec list`（全文搜索仅使用`rg`）
- 决定范围：新增功能 vs 修改现有功能
- 选择唯一的`change-id`：kebab-case格式，动词开头（`add-`，`update-`，`remove-`，`refactor-`）
- 构建：`proposal.md`，`tasks.md`，`design.md`（仅在需要时），以及每个受影响功能的增量规范
- 编写增量：使用`## ADDED|MODIFIED|REMOVED|RENAMED Requirements`；每个需求至少包含一个`#### Scenario:`
- 验证：`openspec validate [change-id] --strict`并修复问题
- 请求审批：在提案获批之前不要开始实施

## 三阶段工作流程

### 第一阶段：创建变更
当您需要以下操作时创建提案：
- 添加功能或特性
- 进行破坏性变更（API，模式）
- 更改架构或模式
- 优化性能（改变行为）
- 更新安全模式

触发条件（示例）：
- "帮助我创建一个变更提案"
- "帮助我规划一个变更"
- "帮助我创建一个提案"
- "我想创建一个规范提案"
- "我想创建一个规范"

宽松匹配指导：
- 包含以下之一：`proposal`，`change`，`spec`
- 以及以下之一：`create`，`plan`，`make`，`start`，`help`

跳过提案的情况：
- 错误修复（恢复预期行为）
- 错别字、格式、注释
- 依赖更新（非破坏性）
- 配置更改
- 现有行为的测试

**工作流程**
1. 查阅`openspec/project.md`，`openspec list`和`openspec list --specs`以了解当前上下文。
2. 选择一个独特的动词引导的`change-id`并在`openspec/changes/<id>/`下构建`proposal.md`，`tasks.md`，可选的`design.md`和规范增量。
3. 使用`## ADDED|MODIFIED|REMOVED Requirements`起草规范增量，每个需求至少包含一个`#### Scenario:`。
4. 运行`openspec validate <id> --strict`并在分享提案之前解决任何问题。

### 第二阶段：实施变更
将这些步骤作为待办事项跟踪，并逐一完成。
1. **阅读proposal.md** - 了解正在构建的内容
2. **阅读design.md**（如果存在）- 审查技术决策
3. **阅读tasks.md** - 获取实施检查列表
4. **按顺序实施任务** - 按顺序完成
5. **确认完成** - 在更新状态之前确保`tasks.md`中的每一项都已完成
6. **更新检查列表** - 所有工作完成后，将每个任务设置为`- [x]`，使列表反映实际情况
7. **审批关卡** - 在提案经过审查和批准之前不要开始实施

### 第三阶段：归档变更
部署后，创建单独的PR来：
- 移动`changes/[name]/` → `changes/archive/YYYY-MM-DD-[name]/`
- 如果功能发生变化，更新`specs/`
- 对于纯工具变更使用`openspec archive <change-id> --skip-specs --yes`（始终显式传递变更ID）
- 运行`openspec validate --strict`以确认归档的变更通过检查

## 任何任务之前

**上下文检查清单：**
- [ ] 阅读`specs/[capability]/spec.md`中的相关规范
- [ ] 检查`changes/`中的待处理变更以发现冲突
- [ ] 阅读`openspec/project.md`了解约定
- [ ] 运行`openspec list`查看活动变更
- [ ] 运行`openspec list --specs`查看现有功能

**创建规范之前：**
- 始终检查功能是否已存在
- 优先修改现有规范而不是创建重复项
- 使用`openspec show [spec]`查看当前状态
- 如果请求不明确，在构建之前询问1-2个澄清问题

### 搜索指导
- 枚举规范：`openspec spec list --long`（或用于脚本的`--json`）
- 枚举变更：`openspec list`（或`openspec change list --json` - 已弃用但可用）
- 显示详情：
  - 规范：`openspec show <spec-id> --type spec`（使用`--json`进行过滤）
  - 变更：`openspec show <change-id> --json --deltas-only`
- 全文搜索（使用ripgrep）：`rg -n "Requirement:|Scenario:" openspec/specs`

## 快速开始

### CLI命令

```bash
# 基本命令
openspec list                  # 列出活动变更
openspec list --specs          # 列出规范
openspec show [item]           # 显示变更或规范
openspec validate [item]       # 验证变更或规范
openspec archive <change-id> [--yes|-y]   # 部署后归档（添加--yes用于非交互式运行）

# 项目管理
openspec init [path]           # 初始化OpenSpec
openspec update [path]         # 更新说明文件

# 交互模式
openspec show                  # 提示选择
openspec validate              # 批量验证模式

# 调试
openspec show [change] --json --deltas-only
openspec validate [change] --strict
```

### 命令标志

- `--json` - 机器可读输出
- `--type change|spec` - 区分项目
- `--strict` - 全面验证
- `--no-interactive` - 禁用提示
- `--skip-specs` - 归档时不更新规范
- `--yes`/`-y` - 跳过确认提示（非交互式归档）

## 目录结构

```
openspec/
├── project.md              # 项目约定
├── specs/                  # 当前真相 - 已构建的内容
│   └── [capability]/       # 单一专注功能
│       ├── spec.md         # 需求和场景
│       └── design.md       # 技术模式
├── changes/                # 提案 - 应该变更的内容
│   ├── [change-name]/
│   │   ├── proposal.md     # 原因、内容、影响
│   │   ├── tasks.md        # 实施检查列表
│   │   ├── design.md       # 技术决策（可选；参见标准）
│   │   └── specs/          # 增量变更
│   │       └── [capability]/
│   │           └── spec.md # ADDED/MODIFIED/REMOVED
│   └── archive/            # 已完成变更
```

## 创建变更提案

### 决策树

```
新请求？
├─ 修复恢复规范行为的错误？ → 直接修复
├─ 错别字/格式/注释？ → 直接修复
├─ 新功能/能力？ → 创建提案
├─ 破坏性变更？ → 创建提案
├─ 架构变更？ → 创建提案
└─ 不清楚？ → 创建提案（更安全）
```

### 提案结构

1. **创建目录：** `changes/[change-id]/`（kebab-case格式，动词开头，唯一）

2. **编写proposal.md：**
```markdown
# 变更：[变更的简要描述]

## 原因
[关于问题/机会的1-2句话]

## 变更内容
- [变更列表]
- [用**BREAKING**标记破坏性变更]

## 影响
- 受影响的规范：[列出功能]
- 受影响的代码：[关键文件/系统]
```

3. **创建规范增量：** `specs/[capability]/spec.md`
```markdown
## ADDED Requirements
### Requirement: New Feature
The system SHALL provide...

#### Scenario: Success case
- **WHEN** user performs action
- **THEN** expected result

## MODIFIED Requirements
### Requirement: Existing Feature
[Complete modified requirement]

## REMOVED Requirements
### Requirement: Old Feature
**Reason**: [Why removing]
**Migration**: [How to handle]
```
如果多个功能受到影响，在`changes/[change-id]/specs/<capability>/spec.md`下创建多个增量文件——每个功能一个。

4. **创建tasks.md：**
```markdown
## 1. 实施
- [ ] 1.1 创建数据库模式
- [ ] 1.2 实现API端点
- [ ] 1.3 添加前端组件
- [ ] 1.4 编写测试
```

5. **在需要时创建design.md：**
如果以下任一情况适用，则创建`design.md`；否则省略：
- 跨切面变更（多个服务/模块）或新架构模式
- 新外部依赖或重大数据模型变更
- 安全、性能或迁移复杂性
- 在编码之前从技术决策中受益的歧义

最小`design.md`框架：
```markdown
## 上下文
[背景、约束、利益相关者]

## 目标 / 非目标
- 目标：[...]
- 非目标：[...]

## 决策
- 决策：[内容及原因]
- 考虑的替代方案：[选项 + 理由]

## 风险 / 权衡
- [风险] → 缓解措施

## 迁移计划
[步骤、回滚]

## 待解决问题
- [...]
```

## 规范文件格式

### 关键：场景格式

**正确**（使用####标题）：
```markdown
#### Scenario: User login success
- **WHEN** valid credentials provided
- **THEN** return JWT token
```

**错误**（不要使用项目符号或粗体）：
```markdown
- **Scenario: User login**  ❌
**Scenario**: User login     ❌
### Scenario: User login      ❌
```

每个需求必须至少有一个场景。

### 需求措辞
- 对规范性需求使用SHALL/MUST（除非有意非规范性，否则避免should/may）

### 增量操作

- `## ADDED Requirements` - 新功能
- `## MODIFIED Requirements` - 更改的行为
- `## REMOVED Requirements` - 已弃用的功能
- `## RENAMED Requirements` - 名称更改

标题与`trim(header)`匹配 - 忽略空格。

#### 何时使用ADDED vs MODIFIED
- ADDED：引入可以独立作为需求的新功能或子功能。当变更是正交的（例如添加"斜杠命令配置"）而不是改变现有需求的语义时，优先使用ADDED。
- MODIFIED：更改现有需求的行为、范围或验收标准。始终粘贴完整、更新的需求内容（标题+所有场景）。归档器将用您在此处提供的内容替换整个需求；部分增量将丢弃以前的详细信息。
- RENAMED：仅在名称更改时使用。如果您还更改行为，请使用RENAMED（名称）加上MODIFIED（内容）引用新名称。

常见陷阱：使用MODIFIED添加新关注点而不包含以前的文本。这会在归档时导致细节丢失。如果您没有明确更改现有需求，请在ADDED下添加新需求。

正确编写MODIFIED需求：
1) 在`openspec/specs/<capability>/spec.md`中找到现有需求。
2) 复制整个需求块（从`### Requirement: ...`到其场景）。
3) 将其粘贴到`## MODIFIED Requirements`下并编辑以反映新行为。
4) 确保标题文本完全匹配（忽略空格）并至少保留一个`#### Scenario:`。

RENAMED示例：
```markdown
## RENAMED Requirements
- FROM: `### Requirement: Login`
- TO: `### Requirement: User Authentication`
```

## 故障排除

### 常见错误

**"变更必须至少有一个增量"**
- 检查`changes/[name]/specs/`是否存在并带有.md文件
- 验证文件是否有操作前缀（## ADDED Requirements）

**"需求必须至少有一个场景"**
- 检查场景是否使用`#### Scenario:`格式（4个井号）
- 不要对场景标题使用项目符号或粗体

**静默场景解析失败**
- 精确格式要求：`#### Scenario: Name`
- 使用调试：`openspec show [change] --json --deltas-only`

### 验证提示

```bash
# 始终使用严格模式进行全面检查
openspec validate [change] --strict

# 调试增量解析
openspec show [change] --json | jq '.deltas'

# 检查特定需求
openspec show [spec] --json -r 1
```

## 理想路径脚本

```bash
# 1) 探索当前状态
openspec spec list --long
openspec list
# 可选全文搜索：
# rg -n "Requirement:|Scenario:" openspec/specs
# rg -n "^#|Requirement:" openspec/changes

# 2) 选择变更ID并构建
CHANGE=add-two-factor-auth
mkdir -p openspec/changes/$CHANGE/{specs/auth}
printf "## Why\n...\n\n## What Changes\n- ...\n\n## Impact\n- ...\n" > openspec/changes/$CHANGE/proposal.md
printf "## 1. Implementation\n- [ ] 1.1 ...\n" > openspec/changes/$CHANGE/tasks.md

# 3) 添加增量（示例）
cat > openspec/changes/$CHANGE/specs/auth/spec.md << 'EOF'
## ADDED Requirements
### Requirement: Two-Factor Authentication
Users MUST provide a second factor during login.

#### Scenario: OTP required
- **WHEN** valid credentials are provided
- **THEN** an OTP challenge is required
EOF

# 4) 验证
openspec validate $CHANGE --strict
```

## 多功能示例

```
openspec/changes/add-2fa-notify/
├── proposal.md
├── tasks.md
└── specs/
    ├── auth/
    │   └── spec.md   # ADDED: Two-Factor Authentication
    └── notifications/
        └── spec.md   # ADDED: OTP email notification
```

auth/spec.md
```markdown
## ADDED Requirements
### Requirement: Two-Factor Authentication
...
```

notifications/spec.md
```markdown
## ADDED Requirements
### Requirement: OTP Email Notification
...
```

## 最佳实践

### 简单优先
- 默认小于100行新代码
- 单文件实现，直到证明不足
- 在没有明确理由的情况下避免框架
- 选择无聊、经过验证的模式

### 复杂性触发器
仅在以下情况下添加复杂性：
- 性能数据显示当前解决方案太慢
- 具体规模要求（>1000用户，>100MB数据）
- 需要抽象的多个已证实用例

### 清晰引用
- 使用`file.ts:42`格式表示代码位置
- 引用规范为`specs/auth/spec.md`
- 链接相关的变更和PR

### 功能命名
- 使用动词-名词：`user-auth`，`payment-capture`
- 每个功能单一目的
- 10分钟理解规则
- 如果描述需要"AND"则拆分

### 变更ID命名
- 使用kebab-case，短小且描述性：`add-two-factor-auth`
- 优先使用动词前缀：`add-`，`update-`，`remove-`，`refactor-`
- 确保唯一性；如果已被占用，追加`-2`，`-3`等。

## 工具选择指南

| 任务 | 工具 | 原因 |
|------|------|-----|
| 按模式查找文件 | Glob | 快速模式匹配 |
| 搜索代码内容 | Grep | 优化的正则表达式搜索 |
| 读取特定文件 | Read | 直接文件访问 |
| 探索未知范围 | Task | 多步骤调查 |

## 错误恢复

### 变更冲突
1. 运行`openspec list`查看活动变更
2. 检查重叠的规范
3. 与变更所有者协调
4. 考虑合并提案

### 验证失败
1. 使用`--strict`标志运行
2. 检查JSON输出以获取详细信息
3. 验证规范文件格式
4. 确保场景格式正确

### 缺少上下文
1. 首先阅读project.md
2. 检查相关规范
3. 查看最近的归档
4. 寻求澄清

## 快速参考

### 阶段指示符
- `changes/` - 已提议，尚未构建
- `specs/` - 已构建和部署
- `archive/` - 已完成变更

### 文件用途
- `proposal.md` - 原因和内容
- `tasks.md` - 实施步骤
- `design.md` - 技术决策
- `spec.md` - 需求和行为

### CLI基本命令
```bash
openspec list              # 正在进行什么？
openspec show [item]       # 查看详情
openspec validate --strict # 是否正确？
openspec archive <change-id> [--yes|-y]  # 标记完成（添加--yes用于自动化）
```

记住：规范是真理。变更是提案。保持它们同步。