use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use walkdir::WalkDir;
use regex::Regex;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    types::*,
    config::ProjectConfig,
    scanner::{ProjectScanner, dependency_analyzer::DependencyAnalyzer, metrics_calculator::MetricsCalculator},
};

/// Scanner for Rust projects
pub struct ArchitectureScanner {
    project_path: PathBuf,
    config: ProjectConfig,
    dependency_analyzer: DependencyAnalyzer,
    metrics_calculator: MetricsCalculator,
}

impl ArchitectureScanner {
    pub fn new<P: AsRef<Path>>(project_path: P, config: ProjectConfig) -> Self {
        let project_path = project_path.as_ref().to_path_buf();
        Self {
            dependency_analyzer: DependencyAnalyzer::new(),
            metrics_calculator: MetricsCalculator::new(),
            project_path,
            config,
        }
    }

    /// Scan the project and return architecture map
    pub async fn scan(&self) -> Result<ArchitectureMap> {
        let start_time = std::time::Instant::now();
        
        // Find all Rust files
        let rust_files = self.find_rust_files()?;
        
        // Parse each file
        let mut nodes = HashMap::new();
        let mut all_dependencies = Vec::new();
        
        for file_path in &rust_files {
            if let Ok(node) = self.parse_rust_file(file_path).await {
                let node_id = node.id.clone();
                nodes.insert(node_id.clone(), node);
            }
        }
        
        // Analyze dependencies
        let edges = self.dependency_analyzer.analyze_dependencies(&nodes)?;
        
        // Calculate metrics
        let metrics = self.metrics_calculator.calculate_architecture_metrics(&nodes, &edges);
        
        // Find circular dependencies
        let circular_dependencies = self.dependency_analyzer.find_circular_dependencies(&edges);
        
        // Calculate totals
        let total_modules = nodes.len();
        let total_lines = nodes.values().map(|n| n.metrics.lines_of_code).sum();
        let average_complexity = if total_modules > 0 {
            nodes.values().map(|n| n.metrics.complexity_score).sum::<f64>() / total_modules as f64
        } else {
            0.0
        };
        
        let architecture = ArchitectureMap {
            nodes,
            edges,
            last_scan: Utc::now(),
            total_modules,
            total_lines,
            average_complexity,
            circular_dependencies,
            metrics,
        };
        
        let duration = start_time.elapsed();
        tracing::info!("Scan completed in {:?}", duration);
        
        Ok(architecture)
    }

    /// Find all Rust files in the project
    fn find_rust_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        for entry in WalkDir::new(&self.project_path)
            .follow_links(self.config.scanning.follow_symlinks)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            // Check if it's a Rust file
            if path.extension().map_or(false, |ext| ext == "rs") {
                // Check exclude patterns
                if self.should_exclude_file(path) {
                    continue;
                }
                
                // Check include patterns
                if !self.should_include_file(path) {
                    continue;
                }
                
                // Check file size
                if let Some(max_size) = self.config.scanning.max_file_size {
                    if let Ok(metadata) = std::fs::metadata(path) {
                        if metadata.len() > max_size as u64 {
                            continue;
                        }
                    }
                }
                
                files.push(path.to_path_buf());
            }
        }
        
        Ok(files)
    }

    /// Check if a file should be excluded
    fn should_exclude_file(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        for pattern in &self.config.scanning.exclude_patterns {
            if glob::Pattern::new(pattern)
                .map(|p| p.matches(&path_str))
                .unwrap_or(false)
            {
                return true;
            }
        }
        
        false
    }

    /// Check if a file should be included
    fn should_include_file(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        if self.config.scanning.include_patterns.is_empty() {
            return true;
        }
        
        for pattern in &self.config.scanning.include_patterns {
            if glob::Pattern::new(pattern)
                .map(|p| p.matches(&path_str))
                .unwrap_or(false)
            {
                return true;
            }
        }
        
        false
    }

    /// Parse a single Rust file
    async fn parse_rust_file(&self, file_path: &Path) -> Result<ArchitectureNode> {
        let content = tokio::fs::read_to_string(file_path).await
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;
        
        let relative_path = file_path.strip_prefix(&self.project_path)
            .unwrap_or(file_path);
        
        let name = self.extract_module_name(file_path, &content);
        let module_type = self.determine_module_type(file_path, &content);
        let dependencies = self.extract_dependencies(&content);
        
        // Calculate metrics
        let metrics = self.metrics_calculator.calculate_node_metrics(&content);
        
        // Extract code elements
        let functions = self.extract_functions(&content);
        let structs = self.extract_structs(&content);
        let enums = self.extract_enums(&content);
        let traits = self.extract_traits(&content);
        
        // Get file metadata
        let metadata = std::fs::metadata(file_path)?;
        let last_modified = metadata.modified()?
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let last_modified = DateTime::from_timestamp(
            last_modified.as_secs() as i64,
            last_modified.subsec_nanos(),
        ).unwrap_or_else(Utc::now);
        
        Ok(ArchitectureNode {
            id: Uuid::new_v4().to_string(),
            name,
            module_type,
            file_path: relative_path.to_string_lossy().to_string(),
            dependencies,
            dependents: Vec::new(), // Will be filled by dependency analyzer
            status: NodeStatus::Active,
            metrics,
            last_modified,
            functions,
            structs,
            enums,
            traits,
            position: None,
        })
    }

    /// Extract module name from file path and content
    fn extract_module_name(&self, file_path: &Path, content: &str) -> String {
        // Try to find module declaration
        let mod_regex = Regex::new(r"pub\s+mod\s+(\w+)|mod\s+(\w+)").unwrap();
        if let Some(captures) = mod_regex.captures(content) {
            return captures.get(1).or_else(|| captures.get(2))
                .unwrap().as_str().to_string();
        }
        
        // Fall back to file name without extension
        file_path.file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string()
    }

    /// Determine module type based on file path and content
    fn determine_module_type(&self, file_path: &Path, content: &str) -> ModuleType {
        let path_str = file_path.to_string_lossy().to_lowercase();
        
        // Check file path patterns
        if path_str.contains("test") || path_str.contains("tests") {
            return ModuleType::Testing;
        }
        
        if path_str.contains("example") || path_str.contains("examples") {
            return ModuleType::Utilities;
        }
        
        if path_str.contains("bench") || path_str.contains("benches") {
            return ModuleType::Performance;
        }
        
        if path_str.contains("config") || path_str.contains("settings") {
            return ModuleType::Configuration;
        }
        
        if path_str.contains("api") || path_str.contains("routes") {
            return ModuleType::API;
        }
        
        if path_str.contains("db") || path_str.contains("database") {
            return ModuleType::Database;
        }
        
        if path_str.contains("network") || path_str.contains("net") {
            return ModuleType::Network;
        }
        
        if path_str.contains("auth") || path_str.contains("security") {
            return ModuleType::Security;
        }
        
        if path_str.contains("log") || path_str.contains("logging") {
            return ModuleType::Logging;
        }
        
        if path_str.contains("monitor") || path_str.contains("metrics") {
            return ModuleType::Monitoring;
        }
        
        // Check content patterns
        if content.contains("async") && content.contains("tokio") {
            return ModuleType::Execution;
        }
        
        if content.contains("serde") && content.contains("Serialize") {
            return ModuleType::DataProcessing;
        }
        
        if content.contains("trait") && content.contains("async") {
            return ModuleType::Integration;
        }
        
        if content.contains("struct") && content.contains("impl") {
            return ModuleType::Core;
        }
        
        // Default to Core
        ModuleType::Core
    }

    /// Extract dependencies from file content
    fn extract_dependencies(&self, content: &str) -> Vec<String> {
        let mut dependencies = Vec::new();
        
        // Match use statements
        let use_regex = Regex::new(r"use\s+crate::([^;]+)").unwrap();
        for captures in use_regex.captures_iter(content) {
            if let Some(dep) = captures.get(1) {
                dependencies.push(dep.as_str().to_string());
            }
        }
        
        // Match mod declarations
        let mod_regex = Regex::new(r"mod\s+(\w+)").unwrap();
        for captures in mod_regex.captures_iter(content) {
            if let Some(dep) = captures.get(1) {
                dependencies.push(dep.as_str().to_string());
            }
        }
        
        dependencies
    }

    /// Extract function information
    fn extract_functions(&self, content: &str) -> Vec<FunctionInfo> {
        let mut functions = Vec::new();
        
        let func_regex = Regex::new(r"(?:pub\s+)?(?:async\s+)?fn\s+(\w+)\s*\([^)]*\)").unwrap();
        for captures in func_regex.captures_iter(content) {
            if let Some(name) = captures.get(1) {
                let func_name = name.as_str();
                let is_public = content.contains(&format!("pub fn {}", func_name));
                let is_async = content.contains(&format!("async fn {}", func_name));
                
                // Count parameters
                let param_regex = Regex::new(&format!(r"fn\s+{}\s*\(([^)]*)\)", regex::escape(func_name))).unwrap();
                let param_count = param_regex.captures(content)
                    .map(|c| c.get(1).unwrap().as_str().split(',').count())
                    .unwrap_or(0);
                
                functions.push(FunctionInfo {
                    name: func_name.to_string(),
                    is_public,
                    is_async,
                    parameter_count: param_count,
                    complexity: 1.0, // Simplified
                    lines_of_code: 1, // Simplified
                    documentation: None,
                    attributes: Vec::new(),
                });
            }
        }
        
        functions
    }

    /// Extract struct information
    fn extract_structs(&self, content: &str) -> Vec<StructInfo> {
        let mut structs = Vec::new();
        
        let struct_regex = Regex::new(r"(?:pub\s+)?struct\s+(\w+)").unwrap();
        for captures in struct_regex.captures_iter(content) {
            if let Some(name) = captures.get(1) {
                let struct_name = name.as_str();
                let is_public = content.contains(&format!("pub struct {}", struct_name));
                
                // Count fields (simplified)
                let field_count = content.matches(&format!("struct {}", struct_name))
                    .count();
                
                structs.push(StructInfo {
                    name: struct_name.to_string(),
                    is_public,
                    field_count,
                    derives: Vec::new(),
                    documentation: None,
                    attributes: Vec::new(),
                    generics: Vec::new(),
                });
            }
        }
        
        structs
    }

    /// Extract enum information
    fn extract_enums(&self, content: &str) -> Vec<EnumInfo> {
        let mut enums = Vec::new();
        
        let enum_regex = Regex::new(r"(?:pub\s+)?enum\s+(\w+)").unwrap();
        for captures in enum_regex.captures_iter(content) {
            if let Some(name) = captures.get(1) {
                let enum_name = name.as_str();
                let is_public = content.contains(&format!("pub enum {}", enum_name));
                
                // Count variants (simplified)
                let variant_count = content.matches(&format!("enum {}", enum_name))
                    .count();
                
                enums.push(EnumInfo {
                    name: enum_name.to_string(),
                    is_public,
                    variant_count,
                    derives: Vec::new(),
                    documentation: None,
                    attributes: Vec::new(),
                    generics: Vec::new(),
                });
            }
        }
        
        enums
    }

    /// Extract trait information
    fn extract_traits(&self, content: &str) -> Vec<TraitInfo> {
        let mut traits = Vec::new();
        
        let trait_regex = Regex::new(r"(?:pub\s+)?trait\s+(\w+)").unwrap();
        for captures in trait_regex.captures_iter(content) {
            if let Some(name) = captures.get(1) {
                let trait_name = name.as_str();
                let is_public = content.contains(&format!("pub trait {}", trait_name));
                
                // Count methods (simplified)
                let method_count = content.matches(&format!("trait {}", trait_name))
                    .count();
                
                traits.push(TraitInfo {
                    name: trait_name.to_string(),
                    is_public,
                    method_count,
                    documentation: None,
                    attributes: Vec::new(),
                    generics: Vec::new(),
                    supertraits: Vec::new(),
                });
            }
        }
        
        traits
    }
}

impl ProjectScanner for ArchitectureScanner {
    fn scan(&self) -> Result<ArchitectureMap> {
        // This is a sync wrapper around the async scan method
        tokio::runtime::Runtime::new()?.block_on(self.scan())
    }

    fn scan_incremental(&self, last_scan: Option<ArchitectureMap>) -> Result<ArchitectureMap> {
        // For now, just do a full scan
        // TODO: Implement incremental scanning
        self.scan()
    }
}
