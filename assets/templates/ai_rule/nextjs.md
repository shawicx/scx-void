## NextJS 项目规则

### 包管理与运行时

1. **必须使用 `pnpm` 作为包管理器**，不得使用 npm / yarn / bun（除非用户明确说明例外）
2. **Node.js 版本要求 22+**（NextJS 14+ 对 Node 版本有严格要求）
3. 包管理脚本统一通过 `pnpm run <script>` 调用

### 项目结构约定（App Router）

遵循 NextJS 14+ App Router 规范，不得擅自调整一级目录：

- `app/` — App Router 根目录
  - `layout.tsx` — 根布局
  - `page.tsx` — 页面组件
  - `[route]/` — 动态路由
  - `api/` — Route Handlers（API 路由）
- `components/` — 通用组件
- `lib/` 或 `src/lib/` — 工具函数与业务逻辑
- `public/` — 静态资源
- `types/` — TypeScript 类型定义

### 构建与测试命令

- `pnpm install` — 安装依赖
- `pnpm dev` — 启动开发服务器
- `pnpm build` — 生产构建
- `pnpm start` — 启动生产服务器
- `pnpm lint` — 代码检查

### 依赖与配置规则

1. 服务端组件（RSC）默认，需要交互的组件显式声明 `'use client'`
2. `next.config.js` 扩展配置，不得重写
3. `tsconfig.json` 合并 compilerOptions，保留 `paths` 别名配置
4. 路由组、动态路由、布局嵌套必须遵循 App Router 约定，不得混入 Pages Router 模式（`pages/` 目录）

### 禁止事项

- 不得在 Server Component 中使用浏览器 API（window、document、localStorage）
- 不得在 Client Component 中直接调用 Node API 或数据库
- 不得擅自将 App Router 切换为 Pages Router
- 不得破坏 `app/layout.tsx` 的根布局结构
- 不得在客户端代码中引入服务端-only 的依赖
