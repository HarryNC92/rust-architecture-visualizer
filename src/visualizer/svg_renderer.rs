use anyhow::Result;
use crate::types::{ArchitectureMap, DependencyEdge, ArchitectureNode, Position};

/// Renders SVG elements for the architecture visualization
pub struct SvgRenderer {
    width: f64,
    height: f64,
}

impl SvgRenderer {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    /// Render the complete SVG for the architecture
    pub fn render_architecture(&self, architecture: &ArchitectureMap) -> Result<String> {
        let mut svg = String::new();
        
        // SVG header
        svg.push_str(&format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg" class="architecture-svg">"#,
            self.width, self.height
        ));
        
        // Add definitions for markers and gradients
        svg.push_str(&self.render_definitions());
        
        // Render dependency arrows
        svg.push_str(&self.render_dependencies(&architecture.edges, &architecture.nodes)?);
        
        // Render module nodes
        svg.push_str(&self.render_modules(&architecture.nodes)?);
        
        // SVG footer
        svg.push_str("</svg>");
        
        Ok(svg)
    }

    /// Render SVG definitions (markers, gradients, etc.)
    fn render_definitions(&self) -> String {
        const GRAY_COLOR: &str = "#6c757d";
        const RED_COLOR: &str = "#dc3545";
        const BLACK_COLOR: &str = "#000000";
        
        format!(
            r#"
            <defs>
                <!-- Arrow markers for dependencies -->
                <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
                    <polygon points="0 0, 10 3.5, 0 7" fill="{}" />
                </marker>
                
                <!-- Arrow marker for circular dependencies -->
                <marker id="circular_arrow" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
                    <polygon points="0 0, 10 3.5, 0 7" fill="{}" />
                </marker>
                
                <!-- Gradients for module types -->
                <linearGradient id="core_gradient" x1="0%" y1="0%" x2="100%" y2="100%">
                    <stop offset="0%" style="stop-color:#e74c3c;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#c0392b;stop-opacity:1" />
                </linearGradient>
                
                <linearGradient id="api_gradient" x1="0%" y1="0%" x2="100%" y2="100%">
                    <stop offset="0%" style="stop-color:#e67e22;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#d35400;stop-opacity:1" />
                </linearGradient>
                
                <linearGradient id="data_gradient" x1="0%" y1="0%" x2="100%" y2="100%">
                    <stop offset="0%" style="stop-color:#3498db;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#2980b9;stop-opacity:1" />
                </linearGradient>
                
                <!-- Drop shadow filter -->
                <filter id="dropshadow" x="-50%" y="-50%" width="200%" height="200%">
                    <feDropShadow dx="2" dy="2" stdDeviation="3" flood-color="{}" flood-opacity="0.3"/>
                </filter>
            </defs>
            "#,
            GRAY_COLOR, RED_COLOR, BLACK_COLOR
        )
    }

    /// Render dependency arrows
    fn render_dependencies(
        &self,
        edges: &[DependencyEdge],
        nodes: &std::collections::HashMap<String, ArchitectureNode>,
    ) -> Result<String> {
        let mut svg = String::new();
        
        for edge in edges {
            if let (Some(from_node), Some(to_node)) = (
                nodes.get(&edge.from),
                nodes.get(&edge.to),
            ) {
                let from_pos = self.get_node_position(from_node);
                let to_pos = self.get_node_position(to_node);
                
                let arrow_id = if edge.is_circular {
                    "circular-arrow"
                } else {
                    "arrowhead"
                };
                
                let color = if edge.is_circular {
                    "#dc3545"
                } else {
                    "#6c757d"
                };
                
                let stroke_width = (edge.strength * 3.0).max(1.0);
                
                // Calculate arrow path with curve for better visualization
                let path = self.calculate_arrow_path(from_pos, to_pos);
                
                svg.push_str(&format!(
                    r#"<path d="{}" stroke="{}" stroke-width="{}" fill="none" marker-end="url(#{})" opacity="0.7" class="dependency-arrow" data-from="{}" data-to="{}"/>"#,
                    path, color, stroke_width, arrow_id, edge.from, edge.to
                ));
            }
        }
        
        Ok(svg)
    }

    /// Render module nodes
    fn render_modules(
        &self,
        nodes: &std::collections::HashMap<String, ArchitectureNode>,
    ) -> Result<String> {
        let mut svg = String::new();
        
        for node in nodes.values() {
            let position = self.get_node_position(node);
            let size = self.calculate_node_size(node);
            
            // Module background
            let gradient_id = self.get_gradient_id(&node.module_type);
            let color = node.module_type.color();
            
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" rx="8" ry="8" fill="url(#{})" filter="url(#dropshadow)" class="module-bg" data-module-id="{}"/>"#,
                position.x - size.width / 2.0,
                position.y - size.height / 2.0,
                size.width,
                size.height,
                gradient_id,
                node.id
            ));
            
            // Module border
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" rx="8" ry="8" fill="none" stroke="{}" stroke-width="2" class="module-border" data-module-id="{}"/>"#,
                position.x - size.width / 2.0,
                position.y - size.height / 2.0,
                size.width,
                size.height,
                color,
                node.id
            ));
            
            // Module title
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" text-anchor="middle" fill="white" font-family="Arial, sans-serif" font-size="14" font-weight="bold" class="module-title" data-module-id="{}">{}</text>"#,
                position.x,
                position.y - 10.0,
                node.id,
                node.name
            ));
            
            // Module type
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" text-anchor="middle" fill="white" font-family="Arial, sans-serif" font-size="10" opacity="0.8" class="module-type" data-module-id="{}">{:?}</text>"#,
                position.x,
                position.y + 5.0,
                node.id,
                node.module_type
            ));
            
            // Metrics
            let metrics_y = position.y + 25.0;
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" text-anchor="middle" fill="white" font-family="Arial, sans-serif" font-size="9" class="module-metrics" data-module-id="{}">{} lines, {:.1} complexity</text>"#,
                position.x,
                metrics_y,
                node.id,
                node.metrics.lines_of_code,
                node.metrics.complexity_score
            ));
        }
        
        Ok(svg)
    }

    /// Get node position (simplified layout algorithm)
    fn get_node_position(&self, node: &ArchitectureNode) -> Position {
        if let Some(pos) = &node.position {
            pos.clone()
        } else {
            // Simple grid layout as fallback
            let index = node.id.chars().map(|c| c as u32).sum::<u32>() as usize;
            let cols = 4;
            let row = index / cols;
            let col = index % cols;
            
            Position {
                x: 100.0 + (col as f64) * 200.0,
                y: 100.0 + (row as f64) * 150.0,
                z: 0.0,
            }
        }
    }

    /// Calculate node size based on metrics
    fn calculate_node_size(&self, node: &ArchitectureNode) -> NodeSize {
        let base_width = 150.0;
        let base_height = 80.0;
        
        // Adjust size based on complexity and lines of code
        let complexity_factor = (node.metrics.complexity_score / 10.0).min(1.0);
        let lines_factor = (node.metrics.lines_of_code as f64 / 1000.0).min(1.0);
        
        let width = base_width + (complexity_factor * 50.0);
        let height = base_height + (lines_factor * 30.0);
        
        NodeSize { width, height }
    }

    /// Get gradient ID for module type
    fn get_gradient_id(&self, module_type: &crate::types::ModuleType) -> &'static str {
        match module_type {
            crate::types::ModuleType::Core => "core-gradient",
            crate::types::ModuleType::API => "api-gradient",
            crate::types::ModuleType::DataProcessing => "data-gradient",
            _ => "core-gradient", // Default
        }
    }

    /// Calculate arrow path with curve
    fn calculate_arrow_path(&self, from: Position, to: Position) -> String {
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance < 50.0 {
            // Straight line for close nodes
            format!("M {} {} L {} {}", from.x, from.y, to.x, to.y)
        } else {
            // Curved path for distant nodes
            let control_x = (from.x + to.x) / 2.0;
            let control_y = (from.y + to.y) / 2.0 - (distance * 0.2);
            
            format!(
                "M {} {} Q {} {} {} {}",
                from.x, from.y, control_x, control_y, to.x, to.y
            )
        }
    }
}

/// Node size information
#[derive(Debug, Clone)]
struct NodeSize {
    width: f64,
    height: f64,
}
