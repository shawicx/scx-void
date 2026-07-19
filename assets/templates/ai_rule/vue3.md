## Vue 3 项目规则

### 包管理与运行时

1. **必须使用 `pnpm` 作为包管理器**，不得使用 npm / yarn / bun（除非用户明确说明例外）
2. **Node.js 版本要求 22+**（在 `.nvmrc` / `package.json#engines` 中维护）
3. 包管理脚本统一通过 `pnpm run <script>` 调用

### 项目结构约定

遵循 Vue 3 + TypeScript + Vite 标准结构，不得擅自调整一级目录：

- `src/components/` — 通用组件（.vue 文件）
- `src/views/` 或 `src/pages/` — 路由级页面
- `src/router/` — Vue Router 路由定义
- `src/stores/` — Pinia 状态管理
- `src/composables/` — 组合式函数（hooks）
- `src/api/` 或 `src/services/` — 接口请求封装
- `src/types/` — TypeScript 类型定义
- `src/assets/` — 静态资源
- `src/utils/` — 工具函数

### 构建与测试命令

- `pnpm install` — 安装依赖
- `pnpm dev` — 启动开发服务器
- `pnpm build` — 生产构建
- `pnpm preview` — 预览构建产物
- `pnpm lint` — 代码检查
- `pnpm test` — 运行测试（如已配置）

### 依赖与配置规则

1. `vite.config.ts` 扩展已有插件，不得重写
2. `tsconfig.json` 合并 compilerOptions，不破坏现有配置
3. Vue 组件必须使用 `<script setup lang="ts">` 语法（除非项目已存在 Options API 代码）
4. **Vue 3 项目严禁误配 Vue 2 语法**（如 filters、Vue.extend、new Vue()）
5. 新增依赖前确认是否为 Vue 3 兼容版本（vue、vue-router、pinia 都需要 v3+）

### 禁止事项

- 不得混入 React 语法（JSX 函数组件、useState、useEffect 等）
- 不得在浏览器代码中引入 node-only API（fs、path、process.server-only）
- 不得擅自将 Pinia 替换为 Vuex 或其他状态库
- 不得破坏 `<script setup>` + Composition API 的一致性
