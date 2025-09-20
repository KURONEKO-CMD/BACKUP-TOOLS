// src/scan.rs
use crate::config::Profile;
use crate::ops;
use crate::types::Mode;
use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

/// 遍历并执行复制（支持 dry-run）
pub fn execute_copy(profile: &Profile, mode: Mode, dry_run: bool) -> Result<()> {
    let src = &profile.src;
    let dest_root = &profile.dest;

    for entry in WalkDir::new(src).follow_links(false) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("! 遍历出错: {e}");
                continue;
            }
        };

        // 只处理文件
        if entry.file_type().is_dir() {
            continue;
        }
        if !entry.file_type().is_file() {
            continue;
        }

        // 任意层级的“文件名排除”（例如 .DS_Store）
        if let Some(name) = entry.path().file_name().and_then(|s| s.to_str()) {
            if profile.exclude.iter().any(|x| x == name) {
                continue;
            }
        }

        // 相对路径（相对于 src）
        let rel = match entry.path().strip_prefix(src) {
            Ok(r) => r,
            Err(_) => continue,
        };

        // 顶层目录排除：相对路径的第一个组件命中 exclude 就跳过
        if relative_first_component(rel)
            .map(|c| profile.exclude.iter().any(|x| x == c))
            .unwrap_or(false)
        {
            continue;
        }

        let dest_path = dest_root.join(rel);

        match mode {
            Mode::Sync if dest_path.exists() => {
                println!("[SKIP] {:?}", rel);
                continue;
            }
            _ => {
                println!("[COPY] {:?}", rel);
                if !dry_run {
                    ops::run_copy(entry.path(), &dest_path, mode)?;
                }
            }
        }
    }

    Ok(())
}

fn relative_first_component(rel: &Path) -> Option<&str> {
    rel.components().next().and_then(|c| c.as_os_str().to_str())
}
