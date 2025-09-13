pub mod html_generator;
pub mod svg_renderer;

use anyhow::Result;
use std::path::Path;
use crate::{
    types::ArchitectureMap,
    config::ProjectConfig,
    scanner::ArchitectureScanner,
};

pub use html_generator::ArchitectureVisualizer;

/// Create a new architecture visualizer
pub fn create_visualizer<P: AsRef<Path>>(
    project_path: P,
    config: ProjectConfig,
) -> Result<ArchitectureVisualizer> {
    let scanner = ArchitectureScanner::new(project_path, config);
    Ok(ArchitectureVisualizer::new(scanner))
}
