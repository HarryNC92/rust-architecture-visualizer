use anyhow::Result;
use std::collections::HashMap;
use crate::{
    types::{ArchitectureMap, ModuleType, NodeStatus, VisualizationSettings, Theme, LayoutType},
    config::ProjectConfig,
    scanner::ArchitectureScanner,
};

/// Main architecture visualizer that generates HTML and handles data
pub struct ArchitectureVisualizer {
    scanner: ArchitectureScanner,
    config: ProjectConfig,
    cached_architecture: Option<ArchitectureMap>,
}

impl ArchitectureVisualizer {
    pub fn new(scanner: ArchitectureScanner) -> Self {
        let config = scanner.config.clone();
        Self {
            scanner,
            config,
            cached_architecture: None,
        }
    }

    /// Get the current architecture data
    pub async fn get_architecture(&self) -> Result<ArchitectureMap> {
        if let Some(ref cached) = self.cached_architecture {
            Ok(cached.clone())
        } else {
            self.scanner.scan_async().await
        }
    }

    /// Refresh the architecture data
    pub async fn refresh(&mut self) -> Result<ArchitectureMap> {
        let architecture = self.scanner.scan_async().await?;
        self.cached_architecture = Some(architecture.clone());
        Ok(architecture)
    }

    /// Get the current configuration
    pub fn get_config(&self) -> &ProjectConfig {
        &self.config
    }

    /// Generate HTML for the architecture visualization
    pub fn generate_html(&self, architecture: &ArchitectureMap) -> Result<String> {
        let settings = &self.config.visualization;
        let project_name = self.config.project.name.as_deref().unwrap_or("Rust Project");
        
        Ok(format!(
            r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - Architecture Visualizer</title>
    <style>
        {}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🏗️ Architecture Visualizer</h1>
            <p>Real-time view of your Rust project architecture</p>
            <div class="controls">
                <button id="refresh-btn" class="btn btn-primary">🔄 Refresh</button>
                <button id="layout-btn" class="btn btn-secondary">📐 Layout</button>
                <button id="theme-btn" class="btn btn-secondary">🎨 Theme</button>
                <button id="fullscreen-btn" class="btn btn-secondary">⛶ Fullscreen</button>
            </div>
        </div>
        
        <div class="stats">
            {}
        </div>
        
        <div class="visualization-container">
            <div class="visualization-panel">
                <div class="legend">
                    {}
                </div>
                <div class="architecture-canvas" id="architecture-canvas">
                    {}
                </div>
            </div>
            
            <div class="details-panel" id="details-panel">
                <div class="details-header">
                    <h3>Module Details</h3>
                    <button id="close-details" class="btn btn-close">×</button>
                </div>
                <div class="details-content" id="details-content">
                    <p>Click on a module to see details</p>
                </div>
            </div>
        </div>
        
        <div class="footer">
            <div class="info">
                <span>Last updated: {}</span>
                <span>Total modules: {}</span>
                <span>Dependencies: {}</span>
            </div>
        </div>
    </div>
    
    <script>
        {}
    </script>
</body>
</html>
        "#,
            project_name,
            self.generate_css(settings),
            self.generate_stats_html(architecture),
            self.generate_legend_html(),
            self.generate_architecture_html(architecture, settings),
            architecture.last_scan.format("%Y-%m-%d %H:%M:%S UTC"),
            architecture.total_modules,
            architecture.edges.len(),
            self.generate_javascript()
        ))
    }

    /// Generate CSS styles
    fn generate_css(&self, settings: &VisualizationSettings) -> String {
        let theme = match settings.theme {
            Theme::Dark => "dark",
            Theme::Light => "light",
            _ => "auto",
        };
        
        format!(
            r#"
        :root {{
            --primary-color: #667eea;
            --secondary-color: #764ba2;
            --success-color: #28a745;
            --warning-color: #ffc107;
            --danger-color: #dc3545;
            --info-color: #17a2b8;
            --light-color: #f8f9fa;
            --dark-color: #343a40;
            --border-color: #dee2e6;
            --shadow: 0 2px 10px rgba(0,0,0,0.1);
            --shadow-lg: 0 10px 25px rgba(0,0,0,0.15);
        }}
        
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, var(--primary-color) 0%, var(--secondary-color) 100%);
            min-height: 100vh;
            color: #333;
        }}
        
        .container {{
            max-width: 1400px;
            margin: 0 auto;
            background: white;
            min-height: 100vh;
            box-shadow: var(--shadow-lg);
        }}
        
        .header {{
            background: linear-gradient(135deg, #2c3e50 0%, #34495e 100%);
            color: white;
            padding: 2rem;
            text-align: center;
        }}
        
        .header h1 {{
            font-size: 2.5rem;
            margin-bottom: 0.5rem;
            font-weight: 300;
        }}
        
        .header p {{
            font-size: 1.1rem;
            opacity: 0.9;
            margin-bottom: 2rem;
        }}
        
        .controls {{
            display: flex;
            gap: 1rem;
            justify-content: center;
            flex-wrap: wrap;
        }}
        
        .btn {{
            padding: 0.75rem 1.5rem;
            border: none;
            border-radius: 25px;
            cursor: pointer;
            font-size: 0.9rem;
            font-weight: 500;
            transition: all 0.3s ease;
            text-decoration: none;
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
        }}
        
        .btn-primary {{
            background: var(--primary-color);
            color: white;
        }}
        
        .btn-primary:hover {{
            background: #5a6fd8;
            transform: translateY(-2px);
        }}
        
        .btn-secondary {{
            background: rgba(255,255,255,0.2);
            color: white;
            border: 1px solid rgba(255,255,255,0.3);
        }}
        
        .btn-secondary:hover {{
            background: rgba(255,255,255,0.3);
            transform: translateY(-2px);
        }}
        
        .btn-close {{
            background: var(--danger-color);
            color: white;
            padding: 0.5rem;
            border-radius: 50%;
            width: 2rem;
            height: 2rem;
            display: flex;
            align-items: center;
            justify-content: center;
        }}
        
        .stats {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1.5rem;
            padding: 2rem;
            background: var(--light-color);
        }}
        
        .stat-card {{
            background: white;
            padding: 1.5rem;
            border-radius: 10px;
            box-shadow: var(--shadow);
            text-align: center;
            transition: transform 0.3s ease;
        }}
        
        .stat-card:hover {{
            transform: translateY(-5px);
        }}
        
        .stat-number {{
            font-size: 2.5rem;
            font-weight: bold;
            color: var(--primary-color);
            margin-bottom: 0.5rem;
        }}
        
        .stat-label {{
            color: #666;
            font-size: 0.9rem;
            text-transform: uppercase;
            letter-spacing: 1px;
        }}
        
        .visualization-container {{
            display: flex;
            min-height: 600px;
        }}
        
        .visualization-panel {{
            flex: 1;
            position: relative;
            background: #f8f9fa;
        }}
        
        .legend {{
            position: absolute;
            top: 1rem;
            right: 1rem;
            background: white;
            padding: 1rem;
            border-radius: 10px;
            box-shadow: var(--shadow);
            z-index: 10;
        }}
        
        .legend h4 {{
            margin-bottom: 1rem;
            color: #333;
        }}
        
        .legend-item {{
            display: flex;
            align-items: center;
            gap: 0.5rem;
            margin-bottom: 0.5rem;
        }}
        
        .legend-color {{
            width: 1rem;
            height: 1rem;
            border-radius: 3px;
        }}
        
        .architecture-canvas {{
            width: 100%;
            height: 100%;
            position: relative;
            overflow: hidden;
        }}
        
        .module-node {{
            position: absolute;
            background: white;
            border-radius: 10px;
            box-shadow: var(--shadow);
            cursor: pointer;
            transition: all 0.3s ease;
            border-left: 4px solid var(--primary-color);
            min-width: 200px;
            max-width: 300px;
        }}
        
        .module-node:hover {{
            transform: translateY(-5px);
            box-shadow: var(--shadow-lg);
        }}
        
        .module-header {{
            padding: 1rem;
            background: linear-gradient(135deg, var(--primary-color) 0%, #5a6fd8 100%);
            color: white;
            border-radius: 10px 10px 0 0;
        }}
        
        .module-title {{
            font-size: 1.1rem;
            font-weight: 600;
            margin-bottom: 0.25rem;
        }}
        
        .module-type {{
            font-size: 0.8rem;
            opacity: 0.9;
            text-transform: uppercase;
            letter-spacing: 1px;
        }}
        
        .module-body {{
            padding: 1rem;
        }}
        
        .module-metrics {{
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 0.75rem;
            margin-bottom: 1rem;
        }}
        
        .metric {{
            text-align: center;
            padding: 0.5rem;
            background: var(--light-color);
            border-radius: 5px;
        }}
        
        .metric-value {{
            font-size: 1.1rem;
            font-weight: bold;
            color: #333;
        }}
        
        .metric-label {{
            font-size: 0.7rem;
            color: #666;
            margin-top: 0.25rem;
        }}
        
        .dependency-arrow {{
            position: absolute;
            pointer-events: none;
            z-index: 1;
        }}
        
        .details-panel {{
            width: 350px;
            background: white;
            border-left: 1px solid var(--border-color);
            display: none;
            flex-direction: column;
        }}
        
        .details-panel.open {{
            display: flex;
        }}
        
        .details-header {{
            padding: 1rem;
            background: var(--light-color);
            border-bottom: 1px solid var(--border-color);
            display: flex;
            justify-content: space-between;
            align-items: center;
        }}
        
        .details-content {{
            flex: 1;
            padding: 1rem;
            overflow-y: auto;
        }}
        
        .footer {{
            background: var(--light-color);
            padding: 1rem 2rem;
            border-top: 1px solid var(--border-color);
        }}
        
        .info {{
            display: flex;
            gap: 2rem;
            font-size: 0.9rem;
            color: #666;
        }}
        
        @media (max-width: 768px) {{
            .visualization-container {{
                flex-direction: column;
            }}
            
            .details-panel {{
                width: 100%;
                height: 300px;
            }}
            
            .controls {{
                flex-direction: column;
                align-items: center;
            }}
        }}
        "#
        )
    }

    /// Generate stats HTML
    fn generate_stats_html(&self, architecture: &ArchitectureMap) -> String {
        let active_modules = architecture.nodes.values()
            .filter(|n| matches!(n.status, NodeStatus::Active))
            .count();
        
        format!(
            r#"
            <div class="stat-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">Total Modules</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">Active Modules</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">Lines of Code</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">{:.1}</div>
                <div class="stat-label">Avg Complexity</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">Dependencies</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">Circular Deps</div>
            </div>
            "#,
            architecture.total_modules,
            active_modules,
            architecture.total_lines,
            architecture.average_complexity,
            architecture.edges.len(),
            architecture.circular_dependencies.len()
        )
    }

    /// Generate legend HTML
    fn generate_legend_html(&self) -> String {
        let module_types = [
            (ModuleType::Core, "Core"),
            (ModuleType::API, "API"),
            (ModuleType::DataProcessing, "Data Processing"),
            (ModuleType::AI, "AI"),
            (ModuleType::Performance, "Performance"),
            (ModuleType::Validation, "Validation"),
            (ModuleType::Execution, "Execution"),
            (ModuleType::Integration, "Integration"),
            (ModuleType::Testing, "Testing"),
            (ModuleType::Utilities, "Utilities"),
        ];

        let legend_items = module_types
            .iter()
            .map(|(module_type, label)| {
                format!(
                    r#"<div class="legend-item">
                        <div class="legend-color" style="background-color: {};"></div>
                        <span>{}</span>
                    </div>"#,
                    module_type.color(),
                    label
                )
            })
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"
            <div class="legend">
                <h4>Module Types</h4>
                {}
            </div>
            "#,
            legend_items
        )
    }

    /// Generate architecture visualization HTML
    fn generate_architecture_html(&self, architecture: &ArchitectureMap, settings: &VisualizationSettings) -> String {
        // Generate SVG for dependencies
        let svg = self.generate_dependency_svg(architecture);
        
        // Generate module nodes
        let nodes = self.generate_module_nodes(architecture, settings);
        
        format!(
            r#"
            <div class="architecture-canvas">
                {}
                {}
            </div>
            "#,
            svg,
            nodes
        )
    }

    /// Generate dependency SVG
    fn generate_dependency_svg(&self, architecture: &ArchitectureMap) -> String {
        if architecture.edges.is_empty() {
            return String::new();
        }

        let mut svg_lines = Vec::new();
        
        for edge in &architecture.edges {
            // This is a simplified version - in a real implementation,
            // you'd calculate actual positions based on the layout algorithm
            let from_pos = (100.0, 100.0); // Placeholder
            let to_pos = (200.0, 200.0);   // Placeholder
            
            let color = if edge.is_circular {
                "#dc3545" // Red for circular dependencies
            } else {
                "#6c757d" // Gray for normal dependencies
            };
            
            svg_lines.push(format!(
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="2" opacity="0.6" marker-end="url(#arrowhead)"/>"#,
                from_pos.0, from_pos.1, to_pos.0, to_pos.1, color
            ));
        }
        
        format!(
            r#"
            <svg class="dependency-arrow" style="width: 100%; height: 100%; position: absolute; top: 0; left: 0;">
                <defs>
                    <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
                        <polygon points="0 0, 10 3.5, 0 7" fill="{}" />
                    </marker>
                </defs>
                {}
            </svg>
            "#,
            "#6c757d", svg_lines.join("")
        )
    }

    /// Generate module nodes
    fn generate_module_nodes(&self, architecture: &ArchitectureMap, settings: &VisualizationSettings) -> String {
        let mut nodes = Vec::new();
        
        for (i, node) in architecture.nodes.values().enumerate() {
            let x = 50 + (i % 4) * 250;
            let y = 100 + (i / 4) * 200;
            
            let color = node.module_type.color();
            let icon = node.module_type.icon();
            
            nodes.push(format!(
                r#"
                <div class="module-node" style="left: {}px; top: {}px;" data-module-id="{}">
                    <div class="module-header" style="border-left-color: {};">
                        <div class="module-title">{} {}</div>
                        <div class="module-type">{:?}</div>
                    </div>
                    <div class="module-body">
                        <div class="module-metrics">
                            <div class="metric">
                                <div class="metric-value">{}</div>
                                <div class="metric-label">Lines</div>
                            </div>
                            <div class="metric">
                                <div class="metric-value">{:.1}</div>
                                <div class="metric-label">Complexity</div>
                            </div>
                            <div class="metric">
                                <div class="metric-value">{}</div>
                                <div class="metric-label">Functions</div>
                            </div>
                            <div class="metric">
                                <div class="metric-value">{}</div>
                                <div class="metric-label">Structs</div>
                            </div>
                        </div>
                    </div>
                </div>
                "#,
                x, y, node.id, color, icon, node.name, node.module_type,
                node.metrics.lines_of_code, node.metrics.complexity_score,
                node.metrics.function_count, node.metrics.struct_count
            ));
        }
        
        nodes.join("")
    }

    /// Generate JavaScript for interactivity
    fn generate_javascript(&self) -> String {
        r#"
        // Module node click handler
        document.addEventListener('click', function(e) {
            const moduleNode = e.target.closest('.module-node');
            if (moduleNode) {
                const moduleId = moduleNode.dataset.moduleId;
                showModuleDetails(moduleId);
            }
        });
        
        // Close details panel
        document.getElementById('close-details').addEventListener('click', function() {
            document.getElementById('details-panel').classList.remove('open');
        });
        
        // Refresh button
        document.getElementById('refresh-btn').addEventListener('click', function() {
            location.reload();
        });
        
        // Layout button
        document.getElementById('layout-btn').addEventListener('click', function() {
            // TODO: Implement layout switching
            alert('Layout switching not implemented yet');
        });
        
        // Theme button
        document.getElementById('theme-btn').addEventListener('click', function() {
            // TODO: Implement theme switching
            alert('Theme switching not implemented yet');
        });
        
        // Fullscreen button
        document.getElementById('fullscreen-btn').addEventListener('click', function() {
            if (document.fullscreenElement) {
                document.exitFullscreen();
            } else {
                document.documentElement.requestFullscreen();
            }
        });
        
        function showModuleDetails(moduleId) {
            const detailsPanel = document.getElementById('details-panel');
            const detailsContent = document.getElementById('details-content');
            
            // TODO: Load actual module details
            detailsContent.innerHTML = `
                <h4>Module Details</h4>
                <p>Module ID: ${moduleId}</p>
                <p>Click on a module to see detailed information.</p>
            `;
            
            detailsPanel.classList.add('open');
        }
        
        // Auto-refresh if enabled
        if (true) { // TODO: Check if auto-refresh is enabled
            setInterval(function() {
                // TODO: Implement auto-refresh
            }, 30000);
        }
        "#.to_string()
    }
}
