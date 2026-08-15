## Python 项目规则

### 包管理与运行时

1. **包管理器遵循项目现状优先级**：`uv` > `poetry` > `pip + requirements.txt`，不得擅自混用或切换
2. **Python 版本必须与 `pyproject.toml` 的 `requires-python` 或 `.python-version` 一致**，不得擅自升级或降级
3. 虚拟环境（`.venv` / `venv`）目录不入库，依赖通过锁文件（`uv.lock` / `poetry.lock`）复现

### 项目结构约定

遵循 Python 标准项目结构，不得擅自调整一级目录：

- `src/<package>/` — 主代码（推荐 src-layout）或 `<package>/`（flat-layout，遵循项目现状）
- `tests/` — 单元测试（镜像包结构）
- `docs/` — 文档（Sphinx / MkDocs 等）
- `scripts/` — 运维 / 辅助脚本
- `pyproject.toml` — 构建 / 依赖 / 工具配置（PEP 621）
- `requirements.txt` / `requirements-dev.txt` — 依赖锁定（若项目使用）
- `.python-version` — Python 版本声明（pyenv）

### 构建与测试命令

- `uv sync` / `poetry install` / `pip install -e .` — 安装依赖（按项目包管理器）
- `uv run python -m <package>` / `python -m <package>` — 运行
- `pytest` — 运行测试
- `pytest tests/<file>` — 运行指定测试
- `ruff check` — 代码检查（如项目使用 ruff）
- `ruff format` — 格式化
- `mypy` / `pyright` — 类型检查（如项目配置）

### 依赖与配置规则

1. **禁止降级依赖版本**，新增依赖优先选择维护活跃、兼容当前 Python 版本的包
2. `pyproject.toml` 修改采取合并策略，不得重写整个文件
3. 依赖分组（dev / docs / test 等）遵循项目现有分组方式，不得擅自重组
4. 类型注解（type hints）遵循项目现状：全量注解项目必须保持注解完整，未注解项目不强制
5. 新增公开函数 / 类必须有 docstring（遵循 Google / NumPy / Sphinx 风格，按项目现状）
6. 配置文件（`.env`、`config.yaml` 等）修改采取合并策略，不破坏现有键值

### 禁止事项

- 不得擅自切换包管理器（`uv` ↔ `poetry` ↔ `pip`）
- 不得擅自升级或降级 `requires-python` 版本
- 不得把虚拟环境目录（`.venv`）或 `__pycache__` 提交入库
- 不得擅自重写 `pyproject.toml` 整个文件
- 不得引入与项目 lint / formatter 冲突的工具（如已有 ruff 再引入 black / flake8）
- 不得在库代码中硬编码绝对路径或环境特定配置
