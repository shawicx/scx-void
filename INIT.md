这是 一份结构化、可扩展、跨平台友好、开发体验舒服的架构。尽量保持每个模块职责单一、未来扩展轻松。

---

## 项目名称 scx-void

工具叫 **scx-void**

---

## 总体架构理念

工具的核心要满足三个特点：

1. **插件式能力扩展**
   项目类型多、配置生成多、系统命令多，未来一定会增加更多脚本与模板，所以架构要能“随便插新功能”。

2. **独立的 Adapter 层做平台差异封装**
   win/mac 的行为差异（关机命令等）最好被隔离到专用模块，而不是散落在业务里。

3. **良好的边界（各司其职）**

   * CLI 层只处理输入与交互
   * Service 层处理具体逻辑
   * Generator 层处理项目模板与配置生成
   * Platform 层处理系统差异
   * Utils 做小工具集

---

## 顶层目录结构

```
scx-voud/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli/              # 所有命令行参数与交互
│   │   ├── mod.rs
│   │   ├── project.rs    # 项目相关功能的子命令
│   │   ├── system.rs     # 系统命令的子命令
│   │   └── utils.rs
│   │
│   ├── services/         # 完整业务逻辑
│   │   ├── mod.rs
│   │   ├── project/      # 项目安装/生成 逻辑
│   │   │   ├── installers/
│   │   │   │   ├── node_ts.rs
│   │   │   │   ├── react.rs
│   │   │   │   ├── vue.rs
│   │   │   │   ├── nestjs.rs
│   │   │   │   ├── nextjs.rs
│   │   │   │   └── tailwind.rs
│   │   │   ├── templates/
│   │   │   │   ├── node_ts/
│   │   │   │   ├── react/
│   │   │   │   └── ...
│   │   │   ├── mod.rs
│   │   │   └── generator.rs
│   │   └── system/       # 系统级命令逻辑
│   │       ├── mod.rs
│   │       └── shutdown.rs
│   │
│   ├── platform/         # 与平台相关的逻辑
│   │   ├── mod.rs
│   │   ├── windows.rs
│   │   └── macos.rs
│   │
│   ├── utils/            # 工具函数库
│   │   ├── fs.rs
│   │   ├── shell.rs
│   │   └── mod.rs
│   │
│   └── errors.rs         # 全局错误定义
│
└── assets/               # 模板文件（raw）
    ├── templates/
    │   ├── node_ts/
    │   ├── react/
    │   ├── vue/
    │   └── ...
```

---

## 关键模块讲解

### 1. CLI 层（`src/cli/`）

负责：

* 命令解析（用 `clap`）
* 选择列表交互（用 `dialoguer`）
* 输入确认
* 子命令路由到不同 service

例如：

```
scx-voud project init
scx-voud system shutdown
scx-voud project add tailwind
```

CLI 层不做任何逻辑，一律调用 services。

---

### 2. Services 层（业务逻辑核心）

#### `services/project/`

处理所有项目类型选择、依赖安装、模板生成等。

内部拆成：

**installers/**
针对不同项目类型写不同 installer，职责分明、易扩展。例如：

* node_ts.rs
* react.rs
* vue.rs
* nextjs.rs
* nestjs.rs
* tailwind.rs

你未来要加 go + gin？只需要加 go_gin.rs。

**generator.rs**
处理模板文件复制、变量注入、配置生成。

**templates/**
存放渲染前的模板文件，你也可以用 `include_str!()` 打包进二进制。

这样项目类型扩展完全不会污染其他模块。

---

### 3. Services 层之 System 模块

负责所有系统命令，例如：

```
scx-voud system shutdown
scx-voud system shutdown --timer=30
```

内部调用 platform 层执行真正的平台命令。

---

### 4. 平台差异抽象（`src/platform/`）

你会遇到：

* 关机命令 mac 是 `sudo shutdown -h +30`
* Windows 是 `shutdown /s /t 1800`
* 路径分隔符差异
* 系统信息读取不同

你可以做这样一个 trait：

```rust
pub trait SystemOps {
    fn shutdown_in(minutes: u32) -> Result<()>;
}
```

然后分别在：

* windows.rs
* macos.rs

实现它。

services/system 调用的永远是 trait，而不是直接执行平台命令，确保代码保持整洁。

Rust 对这类跨平台抽象非常舒服。

---

### 5. Utils 层

常用工具：

* 执行 shell（跨平台要用 `duct` 或 `tokio::process`）
* 文件创建 / 写入
* 路径处理
* 日志包装

通用能力都放这里，不污染业务代码。

---

## 项目扩展路线（未来准备）

这个架构自然支持以下扩展：

1. 插件系统
   每个 installer 都是一个实现了 trait 的插件，将来可以支持用户动态扩展。

2. 自带项目模板生成器
   你可以提供：

   ```
   scx-voud template create my-react-app
   scx-voud template list
   scx-voud template add <github repo>
   ```

3. 更强交互（像 yeoman）
   支持问答式的项目创建向导。

4. 内置自动更新
   用 `self_update` crate。

---
