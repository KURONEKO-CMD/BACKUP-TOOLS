# backup

## English

### Overview
`backup` is a small Rust CLI I built while learning how to structure file sync tools. It reads a profile-based `backup.toml` configuration and copies files from a source directory to a destination directory in either sync (skip existing) or replace (overwrite) mode.

### Getting Started
1. Install Rust and Cargo (<https://rustup.rs/>).
2. Build or run directly with `cargo run`.
3. Initialize a profile (paths are optional at first):
   ```bash
   cargo run -- init --profile my-home
   ```
   Then open `backup.toml` and fill in `src` / `dest` paths when you are ready.

### Daily Use
- Sync mode (only copy new files):
  ```bash
  cargo run -- sync --profile my-home
  ```
- Replace mode (overwrite everything):
  ```bash
  cargo run -- replace --profile my-home
  ```
- Add `--dry-run` to preview actions without copying.

### Development Notes
- Format: `cargo fmt`
- Lint / build: `cargo check`
- Tests (add later): `cargo test`

---

## 中文说明

### 项目简介
`backup` 是一个练习用的 Rust 命令行工具，目标是用 profile 的方式管理备份任务。程序会读取 `backup.toml`，根据配置把文件从源目录复制到目标目录，支持增量同步（已存在就跳过）和全量覆盖两种模式。

### 快速开始
1. 安装 Rust 与 Cargo（<https://rustup.rs/>）。
2. 可以直接使用 `cargo run` 进行构建/运行。
3. 初始化一个 profile（路径可先留空，之后补齐）：
   ```bash
   cargo run -- init --profile my-home
   ```
   之后手动打开 `backup.toml`，填写 `src` / `dest`。

### 常用命令
- 增量同步：
  ```bash
  cargo run -- sync --profile my-home
  ```
- 全量覆盖：
  ```bash
  cargo run -- replace --profile my-home
  ```
- 想先预览可以加上 `--dry-run`。

### 开发提示
- 代码格式化：`cargo fmt`
- 编译检查：`cargo check`
- 单元测试（未来可以补充）：`cargo test`

