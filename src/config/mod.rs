use std::path::{Path, PathBuf};
use anyhow::Result;

pub mod project_config;

pub use project_config::ProjectConfig;

/// Default configuration values
pub const DEFAULT_SCAN_INTERVAL: u64 = 30;
pub const DEFAULT_PORT: u16 = 8000;
pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_THEME: &str = "auto";

/// Configuration file names to look for
pub const CONFIG_FILES: &[&str] = &[
    "rust-arch-viz.toml",
    "rust-arch-viz.yaml",
    "rust-arch-viz.yml",
    "rust-arch-viz.json",
    ".rust-arch-viz.toml",
    ".rust-arch-viz.yaml",
    ".rust-arch-viz.yml",
    ".rust-arch-viz.json",
];

/// Find configuration file in a directory
pub fn find_config_file<P: AsRef<Path>>(dir: P) -> Option<PathBuf> {
    let dir = dir.as_ref();
    
    for config_file in CONFIG_FILES {
        let path = dir.join(config_file);
        if path.exists() {
            return Some(path);
        }
    }
    
    None
}

/// Load configuration from a directory
pub fn load_config<P: AsRef<Path>>(dir: P) -> Result<ProjectConfig> {
    if let Some(config_path) = find_config_file(&dir) {
        ProjectConfig::from_file(&config_path)
    } else {
        Ok(ProjectConfig::default())
    }
}
