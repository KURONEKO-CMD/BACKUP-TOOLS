use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub default_profile: String,
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub src: PathBuf,
    pub dest: PathBuf,
    pub exclude: Vec<String>,
}

pub struct ConfigLocator {
    /// CLI 传入的 --config
    pub cli_path: Option<PathBuf>,
}

impl ConfigLocator {
    pub fn resolve_path(&self) -> PathBuf {
        // 1) CLI 显式指定
        if let Some(p) = &self.cli_path {
            return p.clone();
        }

        // 2) 环境变量

        if let Ok(p) = env::var("MY_BACKUP_CONFIG") {
            if !p.trim().is_empty() {
                return PathBuf::from(p);
            }
        }

        PathBuf::from("backup.toml")
    }
}

impl Config {
    pub fn load_from(path: &Path) -> Option<Self> {
        if !path.exists() {
            return None;
        }

        let data = fs::read_to_string(path).ok()?;
        toml::from_str(&data).ok()
    }

    pub fn save_to(&self, path: &Path) -> std::io::Result<()> {
        if let Some(dir) = path.parent() {
            if !dir.as_os_str().is_empty() {
                fs::create_dir_all(dir)?;
            }
        }

        let toml_str = toml::to_string_pretty(self).expect("序列化失败");
        fs::write(path, toml_str)
    }
}
