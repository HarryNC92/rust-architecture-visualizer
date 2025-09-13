//! # Rust Architecture Visualizer
//!
//! A beautiful, real-time architecture visualizer for Rust projects that automatically
//! discovers and visualizes your codebase structure, dependencies, and relationships.
//!
//! ## Quick Start
//!
//! ```rust
//! use rust_architecture_visualizer::{ArchitectureScanner, Visualizer};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let scanner = ArchitectureScanner::new("./my-project", Default::default());
//!     let architecture = scanner.scan().await?;
//!     
//!     let visualizer = Visualizer::new(scanner);
//!     visualizer.start_server(8080).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod scanner;
pub mod web;
pub mod visualizer;
pub mod types;

// Re-export main types for convenience
pub use config::ProjectConfig;
pub use scanner::ArchitectureScanner;
pub use visualizer::ArchitectureVisualizer;
pub use web::WebServer;
pub use types::*;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration
pub fn default_config() -> ProjectConfig {
    ProjectConfig::default()
}
