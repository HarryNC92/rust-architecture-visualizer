use anyhow::Result;
use std::collections::{HashMap, HashSet};
use crate::types::{ArchitectureNode, DependencyEdge, DependencyType, ModuleType};

/// Analyzes dependencies between modules
pub struct DependencyAnalyzer {
    // Add any state needed for dependency analysis
}

impl DependencyAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// Analyze dependencies between all nodes
    pub fn analyze_dependencies(
        &self,
        nodes: &HashMap<String, ArchitectureNode>,
    ) -> Result<Vec<DependencyEdge>> {
        let mut edges = Vec::new();
        
        for (source_id, source_node) in nodes {
            for dep_name in &source_node.dependencies {
                // Find the target node by name
                if let Some(target_node) = self.find_node_by_name(nodes, dep_name) {
                    let edge = DependencyEdge {
                        from: source_id.clone(),
                        to: target_node.id.clone(),
                        relationship: self.determine_relationship_type(source_node, target_node),
                        strength: self.calculate_dependency_strength(source_node, target_node),
                        is_circular: false, // Will be updated later
                    };
                    edges.push(edge);
                }
            }
        }
        
        // Update circular dependency flags
        self.update_circular_dependencies(&mut edges, nodes);
        
        Ok(edges)
    }

    /// Find a node by its name
    fn find_node_by_name<'a>(
        &self,
        nodes: &'a HashMap<String, ArchitectureNode>,
        name: &str,
    ) -> Option<&'a ArchitectureNode> {
        nodes.values().find(|node| node.name == name)
    }

    /// Determine the type of relationship between two nodes
    fn determine_relationship_type(
        &self,
        source: &ArchitectureNode,
        target: &ArchitectureNode,
    ) -> DependencyType {
        // Simple heuristic based on module types
        match (&source.module_type, &target.module_type) {
            (ModuleType::API, ModuleType::Core) => DependencyType::Uses,
            (ModuleType::Core, ModuleType::DataProcessing) => DependencyType::Uses,
            (ModuleType::Execution, ModuleType::Core) => DependencyType::Uses,
            (ModuleType::Integration, ModuleType::API) => DependencyType::Uses,
            (ModuleType::Testing, _) => DependencyType::Uses,
            _ => DependencyType::DependsOn,
        }
    }

    /// Calculate the strength of dependency between two nodes
    fn calculate_dependency_strength(
        &self,
        source: &ArchitectureNode,
        target: &ArchitectureNode,
    ) -> f64 {
        // Base strength
        let mut strength = 1.0;
        
        // Increase strength based on number of dependencies
        strength += (source.dependencies.len() as f64) * 0.1;
        
        // Increase strength if target is a core module
        if matches!(target.module_type, ModuleType::Core) {
            strength += 0.5;
        }
        
        // Increase strength if source is an API module
        if matches!(source.module_type, ModuleType::API) {
            strength += 0.3;
        }
        
        // Normalize to 0.0 - 1.0 range
        strength.min(1.0)
    }

    /// Update circular dependency flags
    fn update_circular_dependencies(
        &self,
        edges: &mut Vec<DependencyEdge>,
        nodes: &HashMap<String, ArchitectureNode>,
    ) {
        let circular_deps = self.find_circular_dependencies(edges);
        
        for edge in edges.iter_mut() {
            let edge_pair = (edge.from.clone(), edge.to.clone());
            edge.is_circular = circular_deps.iter().any(|cycle| {
                cycle.windows(2).any(|pair| {
                    (pair[0] == edge_pair.0 && pair[1] == edge_pair.1) ||
                    (pair[0] == edge_pair.1 && pair[1] == edge_pair.0)
                })
            });
        }
    }

    /// Find circular dependencies using DFS
    pub fn find_circular_dependencies(&self, edges: &[DependencyEdge]) -> Vec<Vec<String>> {
        let mut graph = HashMap::new();
        
        // Build adjacency list
        for edge in edges {
            graph.entry(edge.from.clone())
                .or_insert_with(Vec::new)
                .push(edge.to.clone());
        }
        
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut cycles = Vec::new();
        
        for node in graph.keys() {
            if !visited.contains(node) {
                let mut path = Vec::new();
                self.dfs_find_cycles(
                    node,
                    &graph,
                    &mut visited,
                    &mut rec_stack,
                    &mut path,
                    &mut cycles,
                );
            }
        }
        
        cycles
    }

    /// DFS helper to find cycles
    fn dfs_find_cycles(
        &self,
        node: &String,
        graph: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node.clone());
        rec_stack.insert(node.clone());
        path.push(node.clone());
        
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    self.dfs_find_cycles(neighbor, graph, visited, rec_stack, path, cycles);
                } else if rec_stack.contains(neighbor) {
                    // Found a cycle
                    if let Some(cycle_start) = path.iter().position(|n| n == neighbor) {
                        let cycle = path[cycle_start..].to_vec();
                        cycles.push(cycle);
                    }
                }
            }
        }
        
        rec_stack.remove(node);
        path.pop();
    }

    /// Calculate dependency metrics
    pub fn calculate_dependency_metrics(
        &self,
        nodes: &HashMap<String, ArchitectureNode>,
        edges: &[DependencyEdge],
    ) -> DependencyMetrics {
        let total_dependencies = edges.len();
        let circular_deps = self.find_circular_dependencies(edges);
        let circular_count = circular_deps.len();
        
        // Calculate dependency density
        let max_possible_edges = nodes.len() * (nodes.len() - 1);
        let density = if max_possible_edges > 0 {
            total_dependencies as f64 / max_possible_edges as f64
        } else {
            0.0
        };
        
        // Calculate average dependencies per node
        let avg_dependencies = if !nodes.is_empty() {
            total_dependencies as f64 / nodes.len() as f64
        } else {
            0.0
        };
        
        // Find most connected nodes
        let mut node_connections = HashMap::new();
        for edge in edges {
            *node_connections.entry(edge.from.clone()).or_insert(0) += 1;
            *node_connections.entry(edge.to.clone()).or_insert(0) += 1;
        }
        
        let most_connected = node_connections
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(node, _)| node.clone());
        
        DependencyMetrics {
            total_dependencies,
            circular_dependencies: circular_count,
            dependency_density: density,
            average_dependencies_per_node: avg_dependencies,
            most_connected_node: most_connected,
        }
    }
}

/// Metrics about dependencies
#[derive(Debug, Clone)]
pub struct DependencyMetrics {
    pub total_dependencies: usize,
    pub circular_dependencies: usize,
    pub dependency_density: f64,
    pub average_dependencies_per_node: f64,
    pub most_connected_node: Option<String>,
}
