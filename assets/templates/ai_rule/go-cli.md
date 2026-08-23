## Go CLI 项目规则

### 构建工具与运行时

1. **必须使用 Go 官方工具链与 Go Modules**（`go.mod` / `go.sum`），不得切换 GOPATH 模式或引入其他包管理方案
2. **Go 语言版本必须与 `go.mod` 中 `go` 指令声明一致**，不得擅自升级或降级
3. CLI 框架遵循项目现状（cobra / urfave-cli / flag 等），不得擅自替换
4. 工具链版本（go、golangci-lint）遵循 `go.mod` / CI 配置 / 项目现状，不得擅自更改

### 项目结构约定

遵循 Go 标准 CLI 项目结构，不得擅自调整一级目录：

- `main.go` 或 `cmd/<name>/main.go` — 入口（保持精简，只做装配与启动）
- `cmd/` — 子命令实现（每个子命令一个文件或包）
- `internal/` — 私有实现代码（外部模块不可导入）
- `pkg/` — 可对外暴露的库代码（可选，遵循项目现状）
- `go.mod` / `go.sum` — 模块清单与校验文件（必须提交）

### 构建与测试命令

- `go build ./...` — 编译全部包
- `go run .` 或 `go run ./cmd/<name>` — 运行
- `go test ./...` — 运行所有测试
- `go test ./... -run <name>` — 运行指定测试
- `go fmt ./...` — 格式化
- `go vet ./...` — 静态检查
- `golangci-lint run` — lint 检查（如项目已配置）

### 依赖与配置规则

1. **禁止降级依赖版本**；`go get -u` 大范围升级需经用户批准
2. `go.mod` 修改采取增量合并策略，不得重写整个文件；`go.sum` 由工具维护，不得手工编辑
3. 公开标识符（导出函数 / 类型 / 常量）必须有 doc comment，且以标识符名称开头
4. 错误处理遵循项目现状（`errors.Is` / `errors.As` / `fmt.Errorf("%w")` 包装 / 自定义 error 类型），不得擅自替换方案
5. 新增依赖优先评估标准库（flag / os / bufio 等）能否满足，避免引入功能重叠的第三方库
6. 日志方案遵循项目现状（`log/slog` / logrus / zap 等），不得擅自替换

### 禁止事项

- 不得手改 `go.sum` 或删除校验条目来"绕过"依赖校验
- 不得使用 `panic` 处理可预期的运行时错误（仅限不可恢复场景）
- 不得忽略 `go vet` / lint 报警强行交付
- 不得绕过 `gofmt` 格式化规则引入个人代码风格
- 不得在 `internal/` 与 `pkg/` 之间随意搬移代码
- 不得在入口 `main.go` 中堆积业务逻辑
