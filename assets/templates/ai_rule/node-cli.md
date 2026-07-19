## Node CLI 项目规则

### 包管理与运行时

1. **必须使用 `pnpm` 作为包管理器**，不得使用 npm / yarn / bun（除非用户明确说明例外）
2. **Node.js 版本要求 22+**（在 `package.json#engines` 中维护，CLI 应明确支持的目标版本）
3. **模块系统统一为 ESM**（`"type": "module"`），不得与 CommonJS 混用

### 项目结构约定

遵循 Node.js + TypeScript CLI 标准结构：

- `src/` — 源码目录
  - `index.ts` 或 `src/main.ts` — 入口
  - `cli.ts` — CLI 命令定义（clap/commander/yargs 等，遵循项目现状）
  - `commands/` — 子命令实现
- `bin/` — 可执行入口（或在 package.json#bin 中指向打包后的文件）
- `dist/` — 打包输出（tsup / esbuild / rollup，遵循项目现状）

### 构建与测试命令

- `pnpm install` — 安装依赖
- `pnpm dev` 或 `pnpm start` — 开发模式运行
- `pnpm build` — 打包构建（tsup / esbuild）
- `pnpm lint` — 代码检查
- `pnpm test` — 运行测试

### 依赖与配置规则

1. `package.json#bin` 必须指向打包后的可执行文件，保持可移植性
2. **bin 不得依赖项目外路径**，所有运行时依赖必须可被 Node 解析
3. `tsup.config.ts` / `esbuild.config.js` 扩展配置，不得重写
4. `tsconfig.json` 的 `target` 至少为 `ES2022`，`module` 为 `ESNext`
5. 新增依赖必须考虑 CLI 的目标 Node 版本兼容性

### 禁止事项

- 不得插入浏览器-only API（DOM、window、document）
- 不得在 CLI 中依赖项目路径外的副作用（如读取固定的家目录文件、写入系统目录）
- 不得擅自将 ESM 切换为 CommonJS（或反之）
- 不得破坏 `bin` 字段与打包输出路径的对应关系
