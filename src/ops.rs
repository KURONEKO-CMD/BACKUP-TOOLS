use crate::types::Mode;
use anyhow::{Context, Ok, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// 执行复制（非 dry-run）
pub fn run_copy(src: &Path, dest: &Path, mode: Mode) -> Result<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).with_context(|| format!("无法创建目标目录: {:?}", parent))?;
    }

    match mode {
        Mode::Sync => {
            if dest.exists() {
                return Ok(());
            }

            fs::copy(src, dest).with_context(|| format!("无法复制 {:?} -> {:?}", src, dest))?;
        }

        Mode::Replace => {
            let tmp_path = tmp_path_for(dest);
            fs::copy(src, &tmp_path)
                .with_context(|| format!("无法写临时文件 {:?} -> {:?}", src, tmp_path))?;
            fs::rename(&tmp_path, dest)
                .with_context(|| format!("无法重命名 {:?} -> {:?}", tmp_path, dest))?;
        }
    }
    Ok(())
}

fn tmp_path_for(dest: &Path) -> PathBuf {
    let mut tmp = dest.to_path_buf();
    let filename = dest.file_name().unwrap_or_default();
    tmp.set_file_name(format!(".{}.tmp", filename.to_string_lossy()));

    tmp
}
