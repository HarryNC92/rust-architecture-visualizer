use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};

/// Available themes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    Auto,
    Custom(String),
}

/// Layout types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutType {
    Grid,
    ForceDirected,
    Hierarchical,
    Circular,
    Custom(String),
}

/// Project configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub project: ProjectSettings,
    pub scanning: ScanningSettings,
    pub visualization: VisualizationSettings,
    pub server: ServerSettings,
}

/// Project-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub authors: Vec<String>,
    pub repository: Option<String>,
}

/// Scanning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanningSettings {
    pub include_tests: bool,
    pub include_examples: bool,
    pub include_benches: bool,
    pub include_docs: bool,
    pub exclude_patterns: Vec<String>,
    pub include_patterns: Vec<String>,
    pub scan_interval: u64,
    pub max_file_size: Option<usize>,
    pub follow_symlinks: bool,
    pub ignore_gitignore: bool,
}

/// Visualization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationSettings {
    pub theme: Theme,
    pub layout: LayoutType,
    pub show_metrics: bool,
    pub show_dependencies: bool,
    pub show_errors: bool,
    pub show_warnings: bool,
    pub group_by_type: bool,
    pub show_file_paths: bool,
    pub show_documentation: bool,
    pub filter_complexity: Option<f64>,
    pub filter_type: Option<String>,
    pub auto_refresh: bool,
    pub refresh_interval: u64,
}

/// Server settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
    pub cors_origins: Vec<String>,
    pub enable_websocket: bool,
    pub enable_compression: bool,
    pub max_request_size: Option<usize>,
    pub timeout: Option<u64>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            project: ProjectSettings {
                name: None,
                description: None,
                version: None,
                authors: Vec::new(),
                repository: None,
            },
            scanning: ScanningSettings {
                include_tests: true,
                include_examples: false,
                include_benches: false,
                include_docs: false,
                exclude_patterns: vec![
                    "target/**".to_string(),
                    "**/target/**".to_string(),
                    "**/.git/**".to_string(),
                    "**/node_modules/**".to_string(),
                    "**/.*".to_string(),
                ],
                include_patterns: vec!["**/*.rs".to_string()],
                scan_interval: 30,
                max_file_size: Some(10 * 1024 * 1024), // 10MB
                follow_symlinks: false,
                ignore_gitignore: true,
            },
            visualization: VisualizationSettings {
                theme: Theme::Auto,
                layout: LayoutType::ForceDirected,
                show_metrics: true,
                show_dependencies: true,
                show_errors: true,
                show_warnings: true,
                group_by_type: true,
                show_file_paths: true,
                show_documentation: true,
                filter_complexity: None,
                filter_type: None,
                auto_refresh: true,
                refresh_interval: 30,
            },
            server: ServerSettings {
                port: 8000,
                host: "127.0.0.1".to_string(),
                cors_origins: vec!["*".to_string()],
                enable_websocket: true,
                enable_compression: true,
                max_request_size: Some(10 * 1024 * 1024), // 10MB
                timeout: Some(30),
            },
        }
    }
}

impl ProjectConfig {
    /// Load configuration from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("toml");
        
        match extension {
            "toml" => toml::from_str(&content)
                .with_context(|| "Failed to parse TOML config"),
            "yaml" | "yml" => serde_yaml::from_str(&content)
                .with_context(|| "Failed to parse YAML config"),
            "json" => serde_json::from_str(&content)
                .with_context(|| "Failed to parse JSON config"),
            _ => Err(anyhow::anyhow!("Unsupported config file format: {}", extension)),
        }
    }
    
    /// Load configuration from a project directory
    pub fn from_project_dir<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let dir = dir.as_ref();
        
        // Look for config files in the directory
        for config_file in crate::config::CONFIG_FILES {
            let path = dir.join(config_file);
            if path.exists() {
                return Self::from_file(&path);
            }
        }
        
        // If no config file found, try to load from Cargo.toml
        let cargo_toml = dir.join("Cargo.toml");
        if cargo_toml.exists() {
            return Self::from_cargo_toml(&cargo_toml);
        }
        
        // Return default config
        Ok(Self::default())
    }
    
    /// Load configuration from Cargo.toml
    pub fn from_cargo_toml<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| "Failed to read Cargo.toml")?;
        
        let cargo_config: CargoConfig = toml::from_str(&content)
            .with_context(|| "Failed to parse Cargo.toml")?;
        
        let mut config = Self::default();
        
        if let Some(package) = cargo_config.package {
            config.project.name = Some(package.name);
            config.project.description = package.description;
            config.project.version = Some(package.version);
            config.project.authors = package.authors.unwrap_or_default();
            config.project.repository = package.repository;
        }
        
        Ok(config)
    }
    
    /// Save configuration to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let content = match path.extension()
            .and_then(|ext| ext.to_str()) {
            Some("toml") => toml::to_string_pretty(self)
                .with_context(|| "Failed to serialize TOML config")?,
            Some("yaml") | Some("yml") => serde_yaml::to_string(self)
                .with_context(|| "Failed to serialize YAML config")?,
            Some("json") => serde_json::to_string_pretty(self)
                .with_context(|| "Failed to serialize JSON config")?,
            _ => return Err(anyhow::anyhow!("Unsupported config file format")),
        };
        
        std::fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;
        
        Ok(())
    }
}

/// Cargo.toml structure for parsing
#[derive(Debug, Deserialize)]
struct CargoConfig {
    package: Option<CargoPackage>,
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    name: String,
    description: Option<String>,
    version: String,
    authors: Option<Vec<String>>,
    repository: Option<String>,
}
