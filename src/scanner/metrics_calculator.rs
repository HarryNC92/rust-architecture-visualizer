use std::collections::HashMap;
use crate::types::{ArchitectureNode, DependencyEdge, NodeMetrics, ArchitectureMetrics};

/// Calculates various metrics for architecture analysis
pub struct MetricsCalculator {
    // Add any state needed for metrics calculation
}

impl MetricsCalculator {
    pub fn new() -> Self {
        Self {}
    }

    /// Calculate metrics for a single node
    pub fn calculate_node_metrics(&self, content: &str) -> NodeMetrics {
        let lines_of_code = self.count_lines_of_code(content);
        let complexity_score = self.calculate_complexity(content);
        let function_count = self.count_functions(content);
        let struct_count = self.count_structs(content);
        let enum_count = self.count_enums(content);
        let trait_count = self.count_traits(content);
        let error_count = self.count_errors(content);
        let warning_count = self.count_warnings(content);
        
        NodeMetrics {
            lines_of_code,
            complexity_score,
            test_coverage: 0.0, // TODO: Implement test coverage calculation
            function_count,
            struct_count,
            enum_count,
            trait_count,
            last_build_time: None, // TODO: Implement build time tracking
            error_count,
            warning_count,
            dependency_count: 0, // Will be updated by dependency analyzer
            dependent_count: 0,  // Will be updated by dependency analyzer
            cyclomatic_complexity: self.calculate_cyclomatic_complexity(content),
            cognitive_complexity: self.calculate_cognitive_complexity(content),
        }
    }

    /// Calculate overall architecture metrics
    pub fn calculate_architecture_metrics(
        &self,
        nodes: &HashMap<String, ArchitectureNode>,
        edges: &[DependencyEdge],
    ) -> ArchitectureMetrics {
        let total_functions = nodes.values().map(|n| n.metrics.function_count).sum();
        let total_structs = nodes.values().map(|n| n.metrics.struct_count).sum();
        let total_enums = nodes.values().map(|n| n.metrics.enum_count).sum();
        let total_traits = nodes.values().map(|n| n.metrics.trait_count).sum();
        
        let complexities: Vec<f64> = nodes.values().map(|n| n.metrics.complexity_score).collect();
        let max_complexity = complexities.iter().cloned().fold(0.0, f64::max);
        let min_complexity = complexities.iter().cloned().fold(f64::INFINITY, f64::min);
        
        let dependency_density = self.calculate_dependency_density(nodes, edges);
        let modularity_score = self.calculate_modularity_score(nodes, edges);
        let maintainability_index = self.calculate_maintainability_index(nodes);
        
        ArchitectureMetrics {
            total_functions,
            total_structs,
            total_enums,
            total_traits,
            max_complexity: if max_complexity.is_finite() { max_complexity } else { 0.0 },
            min_complexity: if min_complexity.is_finite() { min_complexity } else { 0.0 },
            dependency_density,
            modularity_score,
            maintainability_index,
        }
    }

    /// Count lines of code (excluding comments and empty lines)
    fn count_lines_of_code(&self, content: &str) -> usize {
        content
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with("//") && !trimmed.starts_with("/*")
            })
            .count()
    }

    /// Calculate complexity score based on various factors
    fn calculate_complexity(&self, content: &str) -> f64 {
        let mut complexity = 1.0;
        
        // Add complexity for control structures
        complexity += content.matches("if ").count() as f64 * 0.5;
        complexity += content.matches("match ").count() as f64 * 0.8;
        complexity += content.matches("for ").count() as f64 * 0.6;
        complexity += content.matches("while ").count() as f64 * 0.7;
        complexity += content.matches("loop ").count() as f64 * 0.8;
        
        // Add complexity for nested structures
        complexity += content.matches("{{").count() as f64 * 0.1;
        
        // Add complexity for async/await
        if content.contains("async") {
            complexity += 0.5;
        }
        
        // Add complexity for error handling
        complexity += content.matches("?.").count() as f64 * 0.2;
        complexity += content.matches("unwrap()").count() as f64 * 0.1;
        complexity += content.matches("expect(").count() as f64 * 0.1;
        
        complexity
    }

    /// Calculate cyclomatic complexity
    fn calculate_cyclomatic_complexity(&self, content: &str) -> f64 {
        let mut complexity = 1.0; // Base complexity
        
        // Count decision points
        complexity += content.matches("if ").count() as f64;
        complexity += content.matches("match ").count() as f64;
        complexity += content.matches("for ").count() as f64;
        complexity += content.matches("while ").count() as f64;
        complexity += content.matches("loop ").count() as f64;
        complexity += content.matches("&&").count() as f64;
        complexity += content.matches("||").count() as f64;
        
        complexity
    }

    /// Calculate cognitive complexity
    fn calculate_cognitive_complexity(&self, content: &str) -> f64 {
        let mut complexity = 0.0;
        let mut nesting_level: i32 = 0;
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            // Increase nesting level for opening braces
            if trimmed.contains('{') {
                nesting_level += 1;
            }
            
            // Decrease nesting level for closing braces
            if trimmed.contains('}') {
                nesting_level = nesting_level.saturating_sub(1);
            }
            
            // Add complexity based on control structures and nesting
            if trimmed.starts_with("if ") {
                complexity += 1.0 + nesting_level as f64;
            } else if trimmed.starts_with("match ") {
                complexity += 2.0 + nesting_level as f64;
            } else if trimmed.starts_with("for ") || trimmed.starts_with("while ") {
                complexity += 1.0 + nesting_level as f64;
            } else if trimmed.starts_with("loop ") {
                complexity += 1.5 + nesting_level as f64;
            }
        }
        
        complexity
    }

    /// Count functions in the content
    fn count_functions(&self, content: &str) -> usize {
        content.matches("fn ").count()
    }

    /// Count structs in the content
    fn count_structs(&self, content: &str) -> usize {
        content.matches("struct ").count()
    }

    /// Count enums in the content
    fn count_enums(&self, content: &str) -> usize {
        content.matches("enum ").count()
    }

    /// Count traits in the content
    fn count_traits(&self, content: &str) -> usize {
        content.matches("trait ").count()
    }

    /// Count errors in the content (simplified)
    fn count_errors(&self, content: &str) -> usize {
        content.matches("panic!").count() + content.matches("unwrap()").count()
    }

    /// Count warnings in the content (simplified)
    fn count_warnings(&self, content: &str) -> usize {
        content.matches("#[warn(").count()
    }

    /// Calculate dependency density
    fn calculate_dependency_density(
        &self,
        nodes: &HashMap<String, ArchitectureNode>,
        edges: &[DependencyEdge],
    ) -> f64 {
        let node_count = nodes.len();
        if node_count <= 1 {
            return 0.0;
        }
        
        let max_possible_edges = node_count * (node_count - 1);
        edges.len() as f64 / max_possible_edges as f64
    }

    /// Calculate modularity score
    fn calculate_modularity_score(
        &self,
        nodes: &HashMap<String, ArchitectureNode>,
        edges: &[DependencyEdge],
    ) -> f64 {
        if nodes.is_empty() {
            return 0.0;
        }
        
        // Simple modularity score based on module type distribution
        let mut type_counts = std::collections::HashMap::new();
        for node in nodes.values() {
            *type_counts.entry(&node.module_type).or_insert(0) += 1;
        }
        
        let total_nodes = nodes.len() as f64;
        let mut entropy = 0.0;
        
        for count in type_counts.values() {
            let probability = *count as f64 / total_nodes;
            if probability > 0.0 {
                entropy -= probability * probability.log2();
            }
        }
        
        // Normalize to 0-1 range
        entropy / (type_counts.len() as f64).log2().max(1.0)
    }

    /// Calculate maintainability index
    fn calculate_maintainability_index(&self, nodes: &HashMap<String, ArchitectureNode>) -> f64 {
        if nodes.is_empty() {
            return 0.0;
        }
        
        let total_lines = nodes.values().map(|n| n.metrics.lines_of_code).sum::<usize>() as f64;
        let avg_complexity = nodes.values()
            .map(|n| n.metrics.complexity_score)
            .sum::<f64>() / nodes.len() as f64;
        
        // Simple maintainability index (higher is better)
        let lines_factor = (1000.0 / total_lines.max(1.0)).min(1.0);
        let complexity_factor = (10.0 / avg_complexity.max(1.0)).min(1.0);
        
        (lines_factor + complexity_factor) / 2.0
    }
}
