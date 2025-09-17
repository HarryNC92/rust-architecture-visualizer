use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a module in the architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureNode {
    pub id: String,
    pub name: String,
    pub module_type: ModuleType,
    pub file_path: String,
    pub dependencies: Vec<String>,
    pub dependents: Vec<String>,
    pub status: NodeStatus,
    pub metrics: NodeMetrics,
    pub last_modified: DateTime<Utc>,
    pub functions: Vec<FunctionInfo>,
    pub structs: Vec<StructInfo>,
    pub enums: Vec<EnumInfo>,
    pub traits: Vec<TraitInfo>,
    pub position: Option<Position>,
}

/// Position of a node in the visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Types of modules in the architecture
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ModuleType {
    Core,
    DataProcessing,
    AI,
    Performance,
    Validation,
    Execution,
    Integration,
    API,
    Processing,
    Scaffold,
    Testing,
    Utilities,
    Configuration,
    Database,
    Network,
    Security,
    Logging,
    Monitoring,
    Other(String),
}

impl ModuleType {
    pub fn color(&self) -> &'static str {
        match self {
            ModuleType::Core => "#e74c3c",
            ModuleType::DataProcessing => "#3498db",
            ModuleType::AI => "#9b59b6",
            ModuleType::Performance => "#f39c12",
            ModuleType::Validation => "#2ecc71",
            ModuleType::Execution => "#1abc9c",
            ModuleType::Integration => "#34495e",
            ModuleType::API => "#e67e22",
            ModuleType::Processing => "#8e44ad",
            ModuleType::Scaffold => "#16a085",
            ModuleType::Testing => "#f1c40f",
            ModuleType::Utilities => "#95a5a6",
            ModuleType::Configuration => "#7f8c8d",
            ModuleType::Database => "#27ae60",
            ModuleType::Network => "#2980b9",
            ModuleType::Security => "#c0392b",
            ModuleType::Logging => "#8e44ad",
            ModuleType::Monitoring => "#d35400",
            ModuleType::Other(_) => "#bdc3c7",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ModuleType::Core => "⚙️",
            ModuleType::DataProcessing => "📊",
            ModuleType::AI => "🤖",
            ModuleType::Performance => "⚡",
            ModuleType::Validation => "✅",
            ModuleType::Execution => "▶️",
            ModuleType::Integration => "🔗",
            ModuleType::API => "🌐",
            ModuleType::Processing => "⚙️",
            ModuleType::Scaffold => "🏗️",
            ModuleType::Testing => "🧪",
            ModuleType::Utilities => "🛠️",
            ModuleType::Configuration => "⚙️",
            ModuleType::Database => "🗄️",
            ModuleType::Network => "🌐",
            ModuleType::Security => "🔒",
            ModuleType::Logging => "📝",
            ModuleType::Monitoring => "📈",
            ModuleType::Other(_) => "📦",
        }
    }

    /// Human-friendly display name for the module type
    pub fn display_name(&self) -> String {
        match self {
            ModuleType::Core => "Core".to_string(),
            ModuleType::DataProcessing => "Data Processing".to_string(),
            ModuleType::AI => "AI".to_string(),
            ModuleType::Performance => "Performance".to_string(),
            ModuleType::Validation => "Validation".to_string(),
            ModuleType::Execution => "Execution".to_string(),
            ModuleType::Integration => "Integration".to_string(),
            ModuleType::API => "API".to_string(),
            ModuleType::Processing => "Processing".to_string(),
            ModuleType::Scaffold => "Scaffold".to_string(),
            ModuleType::Testing => "Testing".to_string(),
            ModuleType::Utilities => "Utilities".to_string(),
            ModuleType::Configuration => "Configuration".to_string(),
            ModuleType::Database => "Database".to_string(),
            ModuleType::Network => "Network".to_string(),
            ModuleType::Security => "Security".to_string(),
            ModuleType::Logging => "Logging".to_string(),
            ModuleType::Monitoring => "Monitoring".to_string(),
            ModuleType::Other(label) => label.clone(),
        }
    }
}

/// Status of a module
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeStatus {
    Active,
    Inactive,
    Error,
    Building,
    Deprecated,
    Experimental,
}

/// Metrics for a module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetrics {
    pub lines_of_code: usize,
    pub complexity_score: f64,
    pub test_coverage: f64,
    pub function_count: usize,
    pub struct_count: usize,
    pub enum_count: usize,
    pub trait_count: usize,
    pub last_build_time: Option<DateTime<Utc>>,
    pub error_count: usize,
    pub warning_count: usize,
    pub dependency_count: usize,
    pub dependent_count: usize,
    pub cyclomatic_complexity: f64,
    pub cognitive_complexity: f64,
}

/// Information about a function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub is_public: bool,
    pub is_async: bool,
    pub parameter_count: usize,
    pub complexity: f64,
    pub lines_of_code: usize,
    pub documentation: Option<String>,
    pub attributes: Vec<String>,
}

/// Information about a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructInfo {
    pub name: String,
    pub is_public: bool,
    pub field_count: usize,
    pub derives: Vec<String>,
    pub documentation: Option<String>,
    pub attributes: Vec<String>,
    pub generics: Vec<String>,
}

/// Information about an enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumInfo {
    pub name: String,
    pub is_public: bool,
    pub variant_count: usize,
    pub derives: Vec<String>,
    pub documentation: Option<String>,
    pub attributes: Vec<String>,
    pub generics: Vec<String>,
}

/// Information about a trait
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitInfo {
    pub name: String,
    pub is_public: bool,
    pub method_count: usize,
    pub documentation: Option<String>,
    pub attributes: Vec<String>,
    pub generics: Vec<String>,
    pub supertraits: Vec<String>,
}

/// A dependency relationship between modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
    pub relationship: DependencyType,
    pub strength: f64,
    pub is_circular: bool,
}

/// Types of dependencies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DependencyType {
    Uses,
    Implements,
    Extends,
    Imports,
    DependsOn,
    Calls,
    References,
    Contains,
}

/// Complete architecture map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureMap {
    pub nodes: HashMap<String, ArchitectureNode>,
    pub edges: Vec<DependencyEdge>,
    pub last_scan: DateTime<Utc>,
    pub total_modules: usize,
    pub total_lines: usize,
    pub average_complexity: f64,
    pub circular_dependencies: Vec<Vec<String>>,
    pub metrics: ArchitectureMetrics,
}

/// Overall architecture metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureMetrics {
    pub total_functions: usize,
    pub total_structs: usize,
    pub total_enums: usize,
    pub total_traits: usize,
    pub max_complexity: f64,
    pub min_complexity: f64,
    pub dependency_density: f64,
    pub modularity_score: f64,
    pub maintainability_index: f64,
}

// Re-export VisualizationSettings from config
pub use crate::config::project_config::VisualizationSettings;

// Re-export Theme and LayoutType from config
pub use crate::config::project_config::{LayoutType, Theme};
