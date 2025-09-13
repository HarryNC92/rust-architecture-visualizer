pub mod rust_scanner;
pub mod dependency_analyzer;
pub mod metrics_calculator;

use anyhow::Result;
use std::path::Path;
use crate::types::ArchitectureMap;
use crate::config::ProjectConfig;

pub use rust_scanner::ArchitectureScanner;

/// Trait for different types of project scanners
pub trait ProjectScanner {
    fn scan(&self) -> Result<ArchitectureMap>;
    fn scan_incremental(&self, last_scan: Option<ArchitectureMap>) -> Result<ArchitectureMap>;
}

/// Create a scanner for a specific project type
pub fn create_scanner<P: AsRef<Path>>(
    project_path: P,
    config: ProjectConfig,
) -> Result<Box<dyn ProjectScanner + Send + Sync>> {
    let project_path = project_path.as_ref();
    
    // Check if it's a Rust project
    if project_path.join("Cargo.toml").exists() {
        Ok(Box::new(ArchitectureScanner::new(project_path, config)))
    } else {
        Err(anyhow::anyhow!("Unsupported project type. Only Rust projects are currently supported."))
    }
}
