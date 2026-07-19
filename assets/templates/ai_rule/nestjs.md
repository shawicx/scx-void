## NestJS 项目规则

### 包管理与运行时

1. **必须使用 `pnpm` 作为包管理器**，不得使用 npm / yarn / bun（除非用户明确说明例外）
2. **Node.js 版本要求 22+**
3. 包管理脚本统一通过 `pnpm run <script>` 调用

### 项目结构约定（模块化）

遵循 NestJS 模块化架构，不得擅自调整一级目录：

- `src/`
  - `main.ts` — 应用入口（bootstrap）
  - `app.module.ts` — 根模块
  - `modules/` 或 `src/<feature>/` — 按业务领域划分模块
    - `<feature>.module.ts` — 模块定义
    - `<feature>.controller.ts` — 控制器
    - `<feature>.service.ts` — 业务服务
    - `<feature>.dto/` — 数据传输对象
    - `<feature>.entity.ts` — 实体（如使用 TypeORM）
- `test/` — E2E 测试

### 构建与测试命令

- `pnpm install` — 安装依赖
- `pnpm start:dev` — 开发模式（热重载）
- `pnpm build` — 生产构建
- `pnpm lint` — 代码检查
- `pnpm test` — 单元测试
- `pnpm test:e2e` — E2E 测试

### 依赖与配置规则

1. 严格遵循依赖注入（DI）：Service 通过构造函数注入，不得在内部 `new` 依赖
2. **控制器只做参数校验与路由转发，业务逻辑必须放在 Service**
3. DTO 必须使用 class-validator + class-transformer 装饰器进行校验
4. `nest-cli.json` 扩展配置，不得重写
5. `tsconfig.json` 合并 compilerOptions，保留装饰器元数据配置（`emitDecoratorMetadata`、`experimentalDecorators`）
6. 数据库 ORM（TypeORM / Prisma / MikroORM）遵循项目现状，不得擅自替换

### 禁止事项

- 不得在 Controller 中编写业务逻辑
- 不得破坏 DI 容器（如手动 new Service、使用静态方法替代注入）
- 不得在 Service 中直接操作 HTTP 请求/响应对象（应通过 DTO 传递）
- 不得擅自将现有 ORM 替换为其他方案
