mod cli;
mod config;
mod ops;
mod scan;
mod types;

use anyhow::{Result, bail};
use clap::Parser;
use cli::{Cli, Commands, RunArgs};
use config::{Config, ConfigLocator, Profile};
use scan::execute_copy;
use types::Mode;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let locator = ConfigLocator {
        cli_path: cli.config.clone(),
    };
    let cfg_path: PathBuf = locator.resolve_path();

    match cli.command {
        Commands::Init(args) => {
            println!("[INIT] 将写入配置文件 => {:?}", cfg_path);

            let cli::InitArgs {
                profile,
                src,
                dest,
                exclude,
            } = args;
            let profile_name = profile;

            // 已有则加载，没有则新建
            let mut cfg = Config::load_from(&cfg_path).unwrap_or_else(|| Config {
                default_profile: profile_name.clone(), // 这里保留一次 clone 作为默认
                profiles: HashMap::new(),
            });

            let src = src.unwrap_or_default();
            let dest = dest.unwrap_or_default();
            let missing_src = src.as_os_str().is_empty();
            let missing_dest = dest.as_os_str().is_empty();

            // 更新默认 profile（若你不想切换默认，这一行可注释掉）
            cfg.default_profile = profile_name.clone();

            // 插入/覆盖 profile
            cfg.profiles
                .insert(profile_name.clone(), Profile { src, dest, exclude });

            cfg.save_to(&cfg_path)?;
            println!("✅ 已保存到 {:?}", cfg_path);

            if missing_src {
                println!(
                    "⚠️  profile '{}' 暂未填写 src，记得打开 {:?} 手动补上",
                    profile_name, cfg_path
                );
            }

            if missing_dest {
                println!(
                    "⚠️  profile '{}' 暂未填写 dest，记得打开 {:?} 手动补上",
                    profile_name, cfg_path
                );
            }
        }

        Commands::Sync(args) => {
            run_with_mode(&cfg_path, Mode::Sync, args)?;
        }

        Commands::Replace(args) => {
            run_with_mode(&cfg_path, Mode::Replace, args)?;
        }
    }

    Ok(())
}

/// 公共执行入口：读取配置 → 选定 profile → 执行（支持 dry-run）
fn run_with_mode(cfg_path: &Path, mode: Mode, args: RunArgs) -> Result<()> {
    let cfg =
        Config::load_from(cfg_path).expect("找不到配置：请先 `backup init` 或用 --config 指定");

    // unwrap_or 需要一个 String；为了后续还要用 cfg，给 default_profile 做个克隆副本
    let profile_name = args.profile.unwrap_or(cfg.default_profile.clone());

    let profile = cfg
        .profiles
        .get(&profile_name)
        .unwrap_or_else(|| panic!("找不到 profile：{}", profile_name));

    if profile.src.as_os_str().is_empty() {
        bail!(
            "profile '{}' 的 src 未设置，请编辑 {:?} 后再试",
            profile_name,
            cfg_path
        );
    }

    if profile.dest.as_os_str().is_empty() {
        bail!(
            "profile '{}' 的 dest 未设置，请编辑 {:?} 后再试",
            profile_name,
            cfg_path
        );
    }

    println!(
        "[{:?}] config={:?} profile={}",
        mode, cfg_path, profile_name
    );
    println!(
        "源: {:?}\n目标: {:?}\n排除: {:?}",
        profile.src, profile.dest, profile.exclude
    );
    println!("dry_run = {}", args.dry_run);

    execute_copy(profile, mode, args.dry_run)?;
    Ok(())
}
