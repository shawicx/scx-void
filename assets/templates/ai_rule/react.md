## React 项目规则

### 包管理与运行时

1. **必须使用 `pnpm` 作为包管理器**，不得使用 npm / yarn / bun（除非用户明确说明例外）
2. **Node.js 版本要求 22+**（在 `.nvmrc` / `package.json#engines` 中维护）
3. 包管理脚本统一通过 `pnpm run <script>` 调用

### 项目结构约定

遵循 React 18 + TypeScript + Vite 标准结构，不得擅自调整一级目录：

- `src/components/` — 通用组件（.tsx 文件）
- `src/pages/` 或 `src/views/` — 路由级页面
- `src/hooks/` — 自定义 Hooks
- `src/store/` — 状态管理（Zustand / Redux 等，遵循项目现状）
- `src/api/` 或 `src/services/` — 接口请求封装
- `src/types/` — TypeScript 类型定义
- `src/utils/` — 工具函数
- `src/assets/` — 静态资源

### 构建与测试命令

- `pnpm install` — 安装依赖
- `pnpm dev` — 启动开发服务器
- `pnpm build` — 生产构建
- `pnpm preview` — 预览构建产物
- `pnpm lint` — 代码检查
- `pnpm test` — 运行测试（如已配置）

### 依赖与配置规则

1. 组件统一使用函数组件 + Hooks，遵循 JSX/TSX 规范
2. `vite.config.ts` 扩展已有插件，不得重写
3. `tsconfig.json` 合并 compilerOptions，不破坏现有配置
4. React 18 项目必须使用 `createRoot` 而非 `ReactDOM.render`
5. 状态管理库（Zustand / Redux / Jotai 等）遵循项目现状，不得擅自替换

### 禁止事项

- 不得混入 Vue 语法（`<template>` SFC、`ref()`、`computed()` 等）
- 不得在类组件与函数组件之间随意切换风格
- 不得在浏览器代码中引入 node-only API（fs、path 等）
- 不得擅自将现有状态管理库替换为其他方案
