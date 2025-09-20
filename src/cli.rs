use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
/// 备份工具（Backup CLI）
/// - sync: 增量（incremental）
/// - replace: 全覆盖（overwrite）

#[derive(Parser, Debug)]
#[command(name = "backup", version, about = "Simple backup tool (sync/replace)")]

pub struct Cli {
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 初始化配置（initialize config）
    Init(InitArgs),
    /// 增量备份（incremental: exists -> skip）
    Sync(RunArgs),
    /// 全覆盖备份（overwrite: always replace）
    Replace(RunArgs),
}

#[derive(Args, Debug)]
pub struct InitArgs {
    /// 配置档名（profile name），例如：work / home
    #[arg(long)]
    pub profile: String,

    /// 源目录（source directory），可省略以稍后手动填写
    #[arg(long)]
    pub src: Option<PathBuf>,

    /// 目标目录（destination directory），可省略以稍后手动填写
    #[arg(long)]
    pub dest: Option<PathBuf>,

    /// 排除的顶层目录名（exclude list, repeatable）
    #[arg(long, value_name = "NAME")]
    pub exclude: Vec<String>,
}

#[derive(Args, Debug)]
pub struct RunArgs {
    /// 使用的配置档（profile）；不填则用默认
    #[arg(long)]
    pub profile: Option<String>,

    /// 干跑（dry-run）：只显示将要做什么，不实际复制
    #[arg(long)]
    pub dry_run: bool,
}
