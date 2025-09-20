# backup

## English

### Overview
`backup` is a small but usable Rust CLI that copies files from a source directory to a destination directory, driven by a profile in `backup.toml`. It supports two modes: Sync (skip if destination file exists) and Replace (overwrite via temp file + rename).

### Install / Run (Release Binary)
Using the release build is recommended.

- macOS/Linux:
  ```bash
  cargo build --release
  ./target/release/backup --help
  # Install via cargo (recommended)
  cargo install --path .                      # installs to ~/.cargo/bin
  # Or custom location
  cargo install --path . --root ~/.local      # puts binary in ~/.local/bin
  ```
  Manual copy alternative (POSIX only):
  ```bash
  install -m 755 ./target/release/backup ~/.local/bin/
  ```

- Windows (PowerShell):
  ```powershell
  cargo build --release
  .\target\release\backup.exe --help
  # Install via cargo (recommended)
  cargo install --path .                      # installs to %USERPROFILE%\.cargo\bin
  # Or custom location
  cargo install --path . --root $env:USERPROFILE\.local  # binary in %USERPROFILE%\.local\bin
  # Manual copy alternative
  Copy-Item .\target\release\backup.exe -Destination $env:USERPROFILE\.local\bin\
  ```

Ensure the install directory is on PATH:
- macOS/Linux: `~/.cargo/bin` (default) or your custom `.../bin` should be in `PATH`.
- Windows: `%USERPROFILE%\.cargo\bin` (default) or `%USERPROFILE%\.local\bin` in `PATH`.
  You can add permanently in PowerShell: `setx PATH "$env:PATH;$env:USERPROFILE\.local\bin"` (restart terminal).

### First Use
Initialize a profile (paths are optional initially):
```bash
backup init --profile my-home
```
Then open `backup.toml` and fill `src` and `dest`.

You may also point to a custom config file with `--config` or env `MY_BACKUP_CONFIG`.

### Run Backups
- Sync (only copy files not present at destination):
  ```bash
  backup sync --profile my-home
  ```
- Replace (always overwrite using a safe temp-file swap):
  ```bash
  backup replace --profile my-home
  ```
- Preview only:
  ```bash
  backup sync --profile my-home --dry-run
  ```

### Config Notes
- `exclude` matches both:
  - top-level directory names (first path component under `src`), and
  - any file name (e.g., `.DS_Store`).
- The tool only processes regular files; it skips directories and doesn’t follow symlinks.
- Running with missing `src` or `dest` is blocked with a clear error.

### Development
- Format: `cargo fmt`
- Build check: `cargo check`
- Run debug: `cargo run -- <subcommand> ...` (for development only)

### Known Limitations
- No timestamp/size comparison in Sync mode (existence-only skip).
- No pruning of extraneous files in destination.
- Metadata (permissions/ownership/timestamps) are not preserved.
- Single-threaded, simple error handling.
 - Windows note: `Replace` uses a temp file then `rename`. On Windows, `rename` fails if the destination exists; current behavior depends on the filesystem. If you hit an error, removing the destination before replace is a future enhancement we can implement.

---

## 中文说明

### 简介
`backup` 是一个可以直接使用的 Rust 命令行备份工具。通过 `backup.toml` 中的 profile 把文件从源目录复制到目标目录，支持两种模式：增量（目标存在则跳过）与全覆盖（写临时文件后原子替换）。

### 安装与运行（建议使用 Release 二进制）
推荐使用 release 版本。

- macOS/Linux：
  ```bash
  cargo build --release
  ./target/release/backup --help
  # 使用 cargo 安装（推荐）
  cargo install --path .                      # 安装到 ~/.cargo/bin
  # 或自定义安装位置
  cargo install --path . --root ~/.local      # 可执行文件在 ~/.local/bin
  ```
  也可以手动拷贝（仅 POSIX 系统）：
  ```bash
  install -m 755 ./target/release/backup ~/.local/bin/
  ```

- Windows（PowerShell）：
  ```powershell
  cargo build --release
  .\target\release\backup.exe --help
  # 使用 cargo 安装（推荐）
  cargo install --path .                      # 安装到 %USERPROFILE%\.cargo\bin
  # 或自定义安装位置
  cargo install --path . --root $env:USERPROFILE\.local  # 可执行文件在 %USERPROFILE%\.local\bin
  # 手动拷贝
  Copy-Item .\target\release\backup.exe -Destination $env:USERPROFILE\.local\bin\
  ```

确保安装目录已加入 PATH：
- macOS/Linux：`~/.cargo/bin`（默认）或你的自定义 `.../bin`。
- Windows：`%USERPROFILE%\.cargo\bin`（默认）或 `%USERPROFILE%\.local\bin`。
  可以通过 PowerShell 永久添加：`setx PATH "$env:PATH;$env:USERPROFILE\.local\bin"`（重启终端生效）。

### 第一次使用
初始化一个 profile（路径可暂时留空，稍后补齐）：
```bash
backup init --profile my-home
```
随后打开 `backup.toml`，填写 `src` 与 `dest`。也可以通过 `--config` 或环境变量 `MY_BACKUP_CONFIG` 指定自定义配置路径。

### 运行备份
- 增量（存在即跳过）：
  ```bash
  backup sync --profile my-home
  ```
- 全覆盖（使用临时文件 + rename 覆盖）：
  ```bash
  backup replace --profile my-home
  ```
- 预览（不实际复制）：
  ```bash
  backup sync --profile my-home --dry-run
  ```

### 配置说明
- `exclude` 两类规则都会生效：
  - 顶层目录名（`src` 下相对路径的第一个组件），命中则跳过整个目录；
  - 任意文件名（例如 `.DS_Store`）。
- 程序仅处理普通文件，不遍历目录内容本身；不跟随符号链接。
- 若 `src`/`dest` 为空会在运行时直接报错并终止。

### 已知限制
- Sync 模式只按存在与否判断，不比较时间戳/大小。
- 不会清理目标中“多余”的文件。
- 不保留权限/时间戳等元数据。
- 目前为单线程，错误处理简单。
 - Windows 提示：`replace` 模式写临时文件后 `rename`。在 Windows 上若目标已存在，`rename` 可能报错（取决于文件系统）。若遇到问题，后续会考虑在覆盖前先删除目标文件以增强兼容性。
