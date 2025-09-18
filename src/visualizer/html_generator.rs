use crate::{
    config::ProjectConfig,
    scanner::ArchitectureScanner,
    types::{ArchitectureMap, ModuleType, NodeStatus, Theme, VisualizationSettings},
};
use anyhow::Result;
use serde_json::{json, Value};

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
        let project_name = self
            .config
            .project
            .name
            .as_deref()
            .unwrap_or("Rust Project");

        let javascript = self.generate_javascript(architecture, settings)?;
        
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
    <link rel="stylesheet" href="https://unpkg.com/reactflow@11.7.4/dist/style.css">
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🏗️ Architecture Visualizer</h1>
            <p>Real-time view of your Rust project architecture</p>
            <div class="controls">
                <button id="refresh-btn" class="btn btn-primary">🔄 Refresh</button>
                <button id="theme-btn" class="btn btn-secondary">🎨 Theme</button>
                <button id="fullscreen-btn" class="btn btn-secondary">⛶ Fullscreen</button>
            </div>
        </div>
        
        <div class="visualization-controls">
            <div class="control-group">
                <h4>Layout</h4>
                <button id="layout-grid" class="btn btn-secondary active">Grid</button>
                <button id="layout-circular" class="btn btn-secondary">Circular</button>
                <button id="layout-hierarchical" class="btn btn-secondary">Hierarchical</button>
            </div>
            <div class="control-group">
                <h4>Organization</h4>
                <button id="reorder-hierarchical" class="btn btn-secondary active">Hierarchical</button>
                <button id="reorder-grouped" class="btn btn-secondary">Grouped by Type</button>
                <button id="reorder-dependency" class="btn btn-secondary">Dependency Driven</button>
                <button id="reorder-alphabetical" class="btn btn-secondary">Alphabetical</button>
            </div>
            <div class="control-group">
                <button id="legend-toggle" class="btn btn-secondary">📋 Legend</button>
            </div>
        </div>
        
        <div class="visualization-container">
            <div class="visualization-panel">
                <div class="architecture-canvas" id="react-flow-root">
                    {}
                </div>
            </div>
        </div>
        
        <div class="legend">
            {}
        </div>
        
        <div class="stats">
            {}
        </div>
        
        <div class="footer">
            <div class="info">
                <span>Last updated: {}</span>
                <span>Total modules: {}</span>
                <span>Dependencies: {}</span>
            </div>
        </div>
    </div>
    
    <script type="module">
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
            javascript
        ))
    }

    /// Generate CSS styles
    fn generate_css(&self, _settings: &VisualizationSettings) -> String {
        String::from(
            r#":root{--primary:#667eea;--danger:#ef4444;}
*{margin:0;padding:0;box-sizing:border-box;}
body{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;min-height:100vh;background:#f4f5ff;color:#1f2937;padding:2rem;}
body.theme-dark{background:#0f172a;color:#e2e8f0;}
.container{max-width:1320px;margin:0 auto;background:#fff;border-radius:20px;box-shadow:0 12px 26px rgba(15,23,42,.12);overflow:hidden;}
body.theme-dark .container{background:#111827;}
.header{background:linear-gradient(135deg,#667eea,#764ba2);color:#fff;text-align:center;padding:2.2rem 2rem;}
.header h1{font-size:2.4rem;margin-bottom:.6rem;}
.header p{opacity:.85;margin-bottom:2rem;}
.controls{display:flex;flex-wrap:wrap;gap:1rem;justify-content:center;}
.btn{padding:.8rem 1.5rem;border:none;border-radius:999px;font-weight:600;display:inline-flex;align-items:center;gap:.5rem;cursor:pointer;}
.btn-primary{background:linear-gradient(135deg,#667eea,#5a67d8);color:#fff;}
.btn-secondary{background:rgba(255,255,255,.16);border:1px solid rgba(255,255,255,.35);color:#fff;}
.btn-secondary.active{background:linear-gradient(135deg,#667eea,#5a67d8);color:#fff;border-color:#5a67d8;}
body.theme-dark .btn-secondary{background:rgba(30,41,59,.7);border-color:rgba(148,163,184,.4);color:#e2e8f0;}
body.theme-dark .btn-secondary.active{background:linear-gradient(135deg,#667eea,#5a67d8);color:#fff;border-color:#5a67d8;}
.btn-close{background:var(--danger);color:#fff;width:2rem;height:2rem;border-radius:50%;display:flex;align-items:center;justify-content:center;}
.stats{display:grid;grid-template-columns:repeat(auto-fit,minmax(200px,1fr));gap:1.4rem;padding:1.8rem;background:rgba(248,250,252,.9);}
body.theme-dark .stats{background:rgba(15,23,42,.72);}
.stat-card{background:#fff;border-radius:16px;padding:1.4rem;text-align:center;box-shadow:0 10px 24px rgba(15,23,42,.12);}
body.theme-dark .stat-card{background:rgba(30,41,59,.92);color:#e2e8f0;}
.stat-number{font-size:2.2rem;font-weight:700;color:#667eea;}
.stat-label{text-transform:uppercase;font-size:.78rem;letter-spacing:.08em;color:#64748b;}
.visualization-controls{display:flex;flex-wrap:wrap;gap:1rem;padding:1.5rem;background:rgba(248,250,252,.9);border-bottom:1px solid rgba(148,163,184,.25);}
body.theme-dark .visualization-controls{background:rgba(15,23,42,.72);}
.control-group{display:flex;flex-wrap:wrap;gap:.5rem;align-items:center;}
.control-group h4{margin:0;font-size:.9rem;color:#64748b;text-transform:uppercase;letter-spacing:.08em;margin-right:.5rem;}
.control-group .btn{font-size:.85rem;padding:.6rem 1rem;background:rgba(102,126,234,.1);border:1px solid rgba(102,126,234,.3);color:#1f2937;}
.control-group .btn:hover{background:rgba(102,126,234,.2);border-color:rgba(102,126,234,.5);}
body.theme-dark .control-group .btn{background:rgba(30,41,59,.7);border-color:rgba(148,163,184,.4);color:#e2e8f0;}
body.theme-dark .control-group .btn:hover{background:rgba(30,41,59,.9);border-color:rgba(148,163,184,.6);}
.visualization-container{display:grid;grid-template-columns:1fr;min-height:600px;}
.visualization-panel{position:relative;padding:1.5rem;background:linear-gradient(135deg,rgba(102,126,234,.08),rgba(118,75,162,.08));}
.legend{position:fixed;top:50%;right:2rem;transform:translateY(-50%);background:#fff;border-radius:12px;padding:1.5rem;box-shadow:0 20px 40px rgba(15,23,42,.15);z-index:1000;display:none;max-width:280px;max-height:80vh;overflow-y:auto;}
.legend.visible{display:block;}
body.theme-dark .legend{background:rgba(30,41,59,.95);color:#e2e8f0;border:1px solid rgba(148,163,184,.3);}
.legend h4{text-transform:uppercase;font-size:.8rem;letter-spacing:.08em;margin-bottom:.55rem;}
.legend-item{display:flex;align-items:center;gap:.5rem;margin-bottom:.5rem;color:#475569;}
.legend-item:last-child{margin-bottom:0;}
.legend-color{width:.8rem;height:.8rem;border-radius:4px;}
.architecture-canvas{position:relative;height:660px;border-radius:18px;overflow:hidden;background:#fbfbff;border:1px solid rgba(148,163,184,.25);box-shadow:0 14px 28px rgba(15,23,42,.12);}
#react-flow-root{width:100%;height:100%;}
.react-flow__attribution{display:none!important;}
.react-flow__pane{cursor:grab;}
.react-flow__pane.dragging{cursor:grabbing;}
.react-flow__node-module{width:210px;border-radius:16px;border:2px solid rgba(102,126,234,.25);background:#fff;box-shadow:0 10px 22px rgba(15,23,42,.12);transition:transform .2s ease,opacity .2s ease;}
.react-flow__node-module.is-selected{transform:translateY(-3px);border-color:#667eea;}
.react-flow__node-module.is-dimmed{opacity:.35;}
.rf-module-card{position:relative;padding:.9rem 1rem;display:flex;flex-direction:column;gap:.75rem;}
.rf-module-card__header{display:flex;align-items:center;gap:.65rem;border-bottom:1px solid rgba(15,23,42,.1);padding-bottom:.4rem;}
.rf-module-card__icon{font-size:1.45rem;}
.rf-module-card__name{font-weight:600;font-size:1rem;color:#1f2937;}
.rf-module-card__type{font-size:.7rem;text-transform:uppercase;letter-spacing:.08em;color:#64748b;}
.rf-module-card__metrics{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:.65rem;}
.rf-metric{padding:.6rem;border-radius:10px;background:rgba(102,126,234,.12);text-align:center;}
.rf-metric__value{font-weight:600;color:#1f2937;}
.rf-metric__label{font-size:.66rem;text-transform:uppercase;letter-spacing:.07em;color:#64748b;}
.empty-architecture{height:100%;display:flex;flex-direction:column;align-items:center;justify-content:center;text-align:center;gap:.8rem;color:#475569;}
.details-panel{background:#fff;border-left:1px solid rgba(148,163,184,.25);display:none;flex-direction:column;padding:1.2rem;gap:.95rem;}
.details-panel.open{display:flex;}
.details-header{display:flex;justify-content:space-between;align-items:center;}
.details-content{flex:1;overflow-y:auto;display:flex;flex-direction:column;gap:.9rem;}
.details-section h4{text-transform:uppercase;font-size:.76rem;letter-spacing:.08em;margin-bottom:.5rem;color:#1f2937;}
.details-heading{display:flex;align-items:center;gap:.6rem;}
.details-icon{font-size:1.7rem;}
.details-title h3{font-size:1.2rem;margin:0;color:#0f172a;}
.details-meta{font-size:.7rem;letter-spacing:.1em;color:#64748b;}
.details-path{font-family:'Fira Code','Source Code Pro',monospace;font-size:.78rem;color:#475569;word-break:break-word;}
.metric-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(130px,1fr));gap:.65rem;}
.metric-item{background:rgba(248,250,252,.95);border-radius:9px;padding:.65rem;display:flex;flex-direction:column;gap:.28rem;}
.metric-item__label{text-transform:uppercase;font-size:.62rem;letter-spacing:.07em;color:#64748b;}
.metric-item__value{font-weight:600;color:#1f2937;}
.chip-row{display:flex;flex-wrap:wrap;gap:.4rem;}
.chip{padding:.36rem .62rem;border-radius:999px;background:rgba(102,126,234,.16);color:#1f2937;font-size:.7rem;font-weight:600;}
.empty-state{font-size:.82rem;color:#94a3b8;font-style:italic;}
.details-list{list-style:none;display:flex;flex-direction:column;gap:.4rem;color:#475569;}
.details-placeholder{color:#94a3b8;font-size:.85rem;}
.footer{background:rgba(248,250,252,.95);padding:1rem 2rem;border-top:1px solid rgba(148,163,184,.28);}
.info{display:flex;gap:1.3rem;flex-wrap:wrap;font-size:.84rem;color:#64748b;}
@media(max-width:760px){body{padding:1rem;}.header h1{font-size:2rem;}.visualization-panel{padding:1.1rem;}.legend{position:relative;top:auto;right:auto;margin-bottom:1.1rem;}.architecture-canvas{height:520px;}.details-panel{width:100%;position:relative;}.controls{flex-direction:column;}}
"#,
        )
    }

    /// Generate stats HTML
    fn generate_stats_html(&self, architecture: &ArchitectureMap) -> String {
        let active_modules = architecture
            .nodes
            .values()
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
                <h4 style="margin-top: 1rem;">Dependency Types</h4>
                <div class="legend-item">
                    <div class="legend-color" style="background-color: #10b981;"></div>
                    <span>Imports/Use</span>
                </div>
                <div class="legend-item">
                    <div class="legend-color" style="background-color: #f59e0b;"></div>
                    <span>Traits/Impl</span>
                </div>
                <div class="legend-item">
                    <div class="legend-color" style="background-color: #8b5cf6;"></div>
                    <span>Types</span>
                </div>
                <div class="legend-item">
                    <div class="legend-color" style="background-color: #ef4444;"></div>
                    <span>Circular Deps</span>
                </div>
            </div>
            "#,
            legend_items
        )
    }

    /// Generate architecture visualization HTML
    fn generate_architecture_html(
        &self,
        architecture: &ArchitectureMap,
        _settings: &VisualizationSettings,
    ) -> String {
        format!(
            r#"<div id="react-flow-root" data-node-count="{}"></div>"#,
            architecture.nodes.len()
        )
    }

    fn build_react_flow_data(
        &self,
        architecture: &ArchitectureMap,
        settings: &VisualizationSettings,
    ) -> Value {
        let mut ordered_nodes: Vec<_> = architecture.nodes.values().collect();
        ordered_nodes.sort_by(|a, b| a.name.cmp(&b.name));

        let mut node_entries = Vec::new();
        for (index, node) in ordered_nodes.iter().enumerate() {
            let mut entry = json!({
                "id": node.id,
                "name": node.name,
                "icon": node.module_type.icon(),
                "moduleType": node.module_type.display_name(),
                "color": node.module_type.color(),
                "status": format!("{:?}", node.status),
                "filePath": node.file_path,
                "order": index,
                "hierarchyLevel": node.dependencies.len(),
                "dependencies": node.dependencies,
                "dependents": node.dependents,
                "metrics": {
                    "lines_of_code": node.metrics.lines_of_code,
                    "complexity_score": node.metrics.complexity_score,
                    "test_coverage": node.metrics.test_coverage,
                    "function_count": node.metrics.function_count,
                    "struct_count": node.metrics.struct_count,
                    "enum_count": node.metrics.enum_count,
                    "trait_count": node.metrics.trait_count,
                    "dependency_count": node.metrics.dependency_count,
                    "dependent_count": node.metrics.dependent_count,
                    "error_count": node.metrics.error_count,
                    "warning_count": node.metrics.warning_count,
                    "last_build_time": node.metrics.last_build_time.map(|time| time.to_rfc3339()),
                },
                "lastModified": node.last_modified.to_rfc3339(),
            });

            if let Some(position) = node.position.as_ref() {
                if let Some(obj) = entry.as_object_mut() {
                    obj.insert(
                        "position".to_string(),
                        json!({
                            "x": position.x,
                            "y": position.y,
                        }),
                    );
                }
            }

            node_entries.push(entry);
        }

        let mut edge_entries = Vec::new();
        for (index, edge) in architecture.edges.iter().enumerate() {
            let color = if edge.is_circular {
                "#ef4444"
            } else {
                "#94a3b8"
            };
            edge_entries.push(json!({
                "id": format!("edge-{}-{}-{}", edge.from, edge.to, index),
                "source": edge.from,
                "target": edge.to,
                "type": "smoothstep",
                "animated": edge.is_circular,
                "label": format!("{:?}", edge.relationship),
                "data": {
                    "relationship": format!("{:?}", edge.relationship),
                    "strength": edge.strength,
                    "isCircular": edge.is_circular,
                    "color": color,
                },
                "style": {
                    "stroke": color,
                    "strokeWidth": 1.6,
                    "opacity": 0.85,
                }
            }));
        }

        let theme = match &settings.theme {
            Theme::Dark => "dark".to_string(),
            Theme::Light => "light".to_string(),
            Theme::Auto => "auto".to_string(),
            Theme::Custom(value) => value.clone(),
        };

        json!({
            "nodes": node_entries,
            "edges": edge_entries,
            "layout": settings.layout.to_string(),
            "settings": {
                "showMetrics": settings.show_metrics,
                "showDependencies": settings.show_dependencies,
                "theme": theme,
            }
        })
    }

    /// Generate JavaScript for interactivity
    fn generate_javascript(
        &self,
        architecture: &ArchitectureMap,
        settings: &VisualizationSettings,
    ) -> Result<String> {
        let data = self.build_react_flow_data(architecture, settings);
        let serialized = serde_json::to_string(&data)?;
        let template = r#"
import * as React from 'https://esm.sh/react@18.2.0';
import * as ReactDOMClient from 'https://esm.sh/react-dom@18.2.0/client';
import ReactFlow, { Background, Controls, MiniMap, MarkerType, ReactFlowProvider, applyEdgeChanges, applyNodeChanges, Handle, Position } from 'https://esm.sh/reactflow@11.6.0?deps=react@18.2.0,react-dom@18.2.0';

const { createRoot } = ReactDOMClient;
const globalObj = typeof globalThis !== 'undefined' ? globalThis : (typeof window !== 'undefined' ? window : {});
if (!globalObj.React) {
    globalObj.React = React;
}
if (!globalObj.ReactDOM) {
    globalObj.ReactDOM = ReactDOMClient;
}

// Global data - ensure it's always defined
const architectureData = __ARCHITECTURE_DATA__ || {};
const nodesData = Array.isArray(architectureData.nodes)
    ? architectureData.nodes
    : Object.values(architectureData.nodes || {});
const shouldShowDependencies = architectureData?.settings?.showDependencies !== false;
const rawEdges = Array.isArray(architectureData.edges)
    ? architectureData.edges
    : Object.values(architectureData.edges || {});
const edgesData = shouldShowDependencies ? rawEdges : [];
const nodeLookup = new Map(nodesData.map((node, index) => [node.id, { ...node, order: node.order ?? index }]));

const layouts = ['grid', 'circular', 'hierarchical'];
const reorderOptions = ['hierarchical', 'grouped-by-type', 'dependency-driven', 'alphabetical'];
let currentLayoutIndex = Math.max(layouts.indexOf((architectureData.layout || 'grid').toLowerCase()), 0);

// Utility functions
const escapeHtml = (value) => value === null || value === undefined ? '' : String(value).replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;').replace(/'/g,'&#39;');
const formatNumber = (value, digits = 0) => value === null || value === undefined ? '—' : Number(value).toLocaleString(undefined, { maximumFractionDigits: digits });

// Collision detection and resolution
const checkCollision = (pos1, pos2, minDistance = 150) => {
    const dx = pos1.x - pos2.x;
    const dy = pos1.y - pos2.y;
    const distance = Math.sqrt(dx * dx + dy * dy);
    return distance < minDistance;
};

const resolveCollisions = (positions, minDistance = 150) => {
    const positionsArray = Array.from(positions.entries());
    const resolved = new Map();
    
    // Sort by original position to maintain some order
    positionsArray.sort((a, b) => a[1].x - b[1].x);
    
    for (const [id, pos] of positionsArray) {
        let newPos = { ...pos };
        let attempts = 0;
        const maxAttempts = 100;
        const baseDistance = minDistance;
        
        while (attempts < maxAttempts) {
            let hasCollision = false;
            
            for (const [otherId, otherPos] of resolved) {
                if (checkCollision(newPos, otherPos, minDistance)) {
                    hasCollision = true;
                    break;
                }
            }
            
            if (!hasCollision) break;
            
            // Try different strategies for repositioning
            if (attempts < 20) {
                // Strategy 1: Move in a spiral pattern
                const angle = (attempts * 0.5) % (Math.PI * 2);
                const distance = baseDistance + (attempts * 10);
                newPos = {
                    x: pos.x + Math.cos(angle) * distance,
                    y: pos.y + Math.sin(angle) * distance
                };
            } else if (attempts < 50) {
                // Strategy 2: Move in a grid pattern
                const gridSize = Math.ceil(Math.sqrt(attempts - 20));
                const gridX = (attempts - 20) % gridSize;
                const gridY = Math.floor((attempts - 20) / gridSize);
                newPos = {
                    x: pos.x + (gridX - gridSize/2) * baseDistance,
                    y: pos.y + (gridY - gridSize/2) * baseDistance
                };
            } else {
                // Strategy 3: Random placement with increasing distance
                const angle = Math.random() * Math.PI * 2;
                const distance = baseDistance + (attempts - 50) * 20;
                newPos = {
                    x: pos.x + Math.cos(angle) * distance,
                    y: pos.y + Math.sin(angle) * distance
                };
            }
            
            attempts++;
        }
        
        resolved.set(id, newPos);
    }
    
    return resolved;
};

const computePositions = (layout, nodes, reorderType = 'hierarchical') => {
    const positions = new Map();
    const total = nodes.length || 1;
    
    // Apply reordering first
    let orderedNodes = [...nodes];
    if (reorderType === 'hierarchical') {
        // Already hierarchical by dependencies
        orderedNodes = nodes;
    } else if (reorderType === 'grouped-by-type') {
        // Group by module type
        const typeGroups = {};
        nodes.forEach(node => {
            const type = node.moduleType || 'Unknown';
            if (!typeGroups[type]) typeGroups[type] = [];
            typeGroups[type].push(node);
        });
        orderedNodes = Object.values(typeGroups).flat();
    } else if (reorderType === 'dependency-driven') {
        // Sort by dependency count (most dependent first)
        orderedNodes = nodes.sort((a, b) => {
            const aDeps = (a.dependencies || []).length;
            const bDeps = (b.dependencies || []).length;
            return bDeps - aDeps;
        });
    } else if (reorderType === 'alphabetical') {
        // Sort alphabetically
        orderedNodes = nodes.sort((a, b) => a.name.localeCompare(b.name));
    }
    
    if (layout === 'circular' && total > 1) {
        const radius = 220 + total * 12;
        const cx = radius + 180;
        const cy = radius * 0.55 + 150;
        orderedNodes.forEach((node, index) => {
            const angle = (Math.PI * 2 * index) / total;
            positions.set(node.id, { x: cx + Math.cos(angle) * radius, y: cy + Math.sin(angle) * radius * 0.7 });
        });
    } else if (layout === 'hierarchical' && total > 1) {
        // Create a hierarchical layout based on dependencies
        const nodeMap = new Map(orderedNodes.map(node => [node.id, node]));
        const levels = new Map();
        const visited = new Set();
        
        // Find root nodes (nodes with no dependencies)
        const rootNodes = orderedNodes.filter(node => 
            !node.dependencies || node.dependencies.length === 0
        );
        
        // Assign levels based on dependency depth
        const assignLevel = (nodeId, level = 0) => {
            if (visited.has(nodeId)) return;
            visited.add(nodeId);
            
            if (!levels.has(level)) levels.set(level, []);
            levels.get(level).push(nodeId);
            
            const node = nodeMap.get(nodeId);
            if (node && node.dependents) {
                node.dependents.forEach(dependentId => {
                    assignLevel(dependentId, level + 1);
                });
            }
        };
        
        rootNodes.forEach(root => assignLevel(root.id));
        
        // Position nodes by level
        const levelHeight = 250;
        const nodeWidth = 300;
        const startX = 150;
        
        levels.forEach((levelNodes, level) => {
            const levelY = 150 + level * levelHeight;
            const spacing = Math.max(350, (window.innerWidth - 300) / Math.max(1, levelNodes.length - 1));
            
            levelNodes.forEach((nodeId, index) => {
                const x = startX + index * spacing;
                positions.set(nodeId, { x, y: levelY });
            });
        });
    } else if (reorderType === 'grouped-by-type') {
        // Group by type layout with proper spacing
        const typeGroups = {};
        orderedNodes.forEach(node => {
            const type = node.moduleType || 'Unknown';
            if (!typeGroups[type]) typeGroups[type] = [];
            typeGroups[type].push(node);
        });
        
        const types = Object.keys(typeGroups);
        const groupHeight = 300;
        const groupWidth = 400;
        const startX = 150;
        const startY = 150;
        const groupSpacing = 500;
        
        types.forEach((type, typeIndex) => {
            const groupNodes = typeGroups[type];
            const columns = Math.ceil(Math.sqrt(groupNodes.length));
            const nodeSpacing = 250;
            
            groupNodes.forEach((node, nodeIndex) => {
                const column = nodeIndex % columns;
                const row = Math.floor(nodeIndex / columns);
                const x = startX + typeIndex * groupSpacing + column * nodeSpacing;
                const y = startY + row * nodeSpacing;
                positions.set(node.id, { x, y });
            });
        });
            } else {
        // Default grid layout with better spacing
        const columns = Math.ceil(Math.sqrt(total));
        const nodeWidth = 300;
        const nodeHeight = 200;
        const padding = 100;
        const spacing = 200;
        
        orderedNodes.forEach((node, index) => {
            const column = index % columns;
            const row = Math.floor(index / columns);
            positions.set(node.id, {
                x: padding + column * (nodeWidth + spacing),
                y: padding + row * (nodeHeight + spacing)
            });
        });
    }
    
    // Apply collision resolution to prevent overlapping
    return resolveCollisions(positions);
};

const buildNodes = (layout, nodes, reorderType = 'hierarchical') => {
    const positions = computePositions(layout, nodes, reorderType);
    return nodes.map((node) => ({
        id: node.id,
        type: 'module',
        position: positions.get(node.id) || { x: 0, y: 0 },
        data: { ...node },
        className: '',
        sourcePosition: Position.Right,
        targetPosition: Position.Left,
        draggable: true
    }));
};

const buildEdges = (edges) => edges
    .map((edge, index) => {
        const source = edge?.source ?? edge?.from;
        const target = edge?.target ?? edge?.to;

        if (!source || !target) {
            console.warn('[Flow] Skipping edge with missing endpoint', edge);
            return null;
        }

        // Determine edge style based on relationship type
        const relationship = edge?.relationship || edge?.label || 'dependency';
        const isCircular = edge?.is_circular || false;
        const strength = edge?.strength || 1;
        
        let edgeStyle = {
            strokeWidth: Math.max(1, strength * 2),
            stroke: '#667eea',
            strokeDasharray: isCircular ? '5,5' : '0',
            markerEnd: 'url(#arrowhead)',
            ...(edge.style || {})
        };
        
        // Color coding based on relationship type
        if (relationship.includes('import') || relationship.includes('use')) {
            edgeStyle.stroke = '#10b981'; // Green for imports
        } else if (relationship.includes('trait') || relationship.includes('impl')) {
            edgeStyle.stroke = '#f59e0b'; // Orange for traits
        } else if (relationship.includes('struct') || relationship.includes('enum')) {
            edgeStyle.stroke = '#8b5cf6'; // Purple for types
        } else if (isCircular) {
            edgeStyle.stroke = '#ef4444'; // Red for circular dependencies
        }
        
        return {
            ...edge,
            id: edge?.id ?? `edge-${source}-${target}-${index}`,
            source,
            target,
            type: 'smoothstep',
            animated: isCircular,
            style: edgeStyle,
            label: relationship,
            labelStyle: { 
                fill: '#1f2937', 
                fontSize: 10, 
                fontWeight: 500,
                textAnchor: 'middle'
            },
            labelBgPadding: [4, 2],
            labelBgBorderRadius: 4,
            labelBgStyle: { 
                fill: 'rgba(255,255,255,0.9)',
                stroke: edgeStyle.stroke,
                strokeWidth: 1
            }
        };
    })
    .filter(Boolean);

// React components
const e = React.createElement;

const ModuleNode = ({ data }) => {
    const metrics = data?.metrics || {};
    const showMetrics = architectureData?.settings?.showMetrics === true;
    const accentColor = data?.color || '#4b5563';
    const handleStyle = {
        width: 12,
        height: 12,
        borderRadius: '50%',
        background: accentColor,
        border: '2px solid #fff',
        boxShadow: '0 0 0 2px rgba(148, 163, 184, 0.25)',
        zIndex: 10
    };

    return e('div', { className: 'rf-module-card' },
        e(Handle, { type: 'target', position: Position.Left, style: handleStyle, isConnectable: false }),
        e(Handle, { type: 'source', position: Position.Right, style: handleStyle, isConnectable: false }),
        e('div', { className: 'rf-module-card__header' },
            e('div', { className: 'rf-module-card__icon' }, data?.icon || ''),
            e('div', null,
                e('div', { className: 'rf-module-card__name' }, data?.name || ''),
                e('div', { className: 'rf-module-card__type' }, data?.moduleType || '')
            )
        ),
        showMetrics ? e('div', { className: 'rf-module-card__metrics' },
            e('div', { className: 'rf-metric' },
                e('div', { className: 'rf-metric__value' }, formatNumber(metrics.lines_of_code)),
                e('div', { className: 'rf-metric__label' }, 'Lines')
            ),
            e('div', { className: 'rf-metric' },
                e('div', { className: 'rf-metric__value' }, formatNumber(metrics.function_count)),
                e('div', { className: 'rf-metric__label' }, 'Funcs')
            ),
            e('div', { className: 'rf-metric' },
                e('div', { className: 'rf-metric__value' }, formatNumber(metrics.complexity_score,1)),
                e('div', { className: 'rf-metric__label' }, 'Complexity')
            ),
            e('div', { className: 'rf-metric' },
                e('div', { className: 'rf-metric__value' }, formatNumber(metrics.dependency_count)),
                e('div', { className: 'rf-metric__label' }, 'Deps')
            )
        ) : null
    );
};

const FlowApp = () => {
    const [layout, setLayout] = React.useState(layouts[currentLayoutIndex] || 'grid');
    const [reorderType, setReorderType] = React.useState('hierarchical');
    const [nodes, setNodes] = React.useState(() =>
        nodesData.length ? buildNodes(layouts[currentLayoutIndex] || 'grid', nodesData, 'hierarchical') : []
    );
    const [edges, setEdges] = React.useState(() =>
        edgesData.length ? buildEdges(edgesData) : []
    );
    const nodeTypes = React.useMemo(() => ({ module: ModuleNode }), []);

    React.useEffect(() => {
        setNodes(nodesData.length ? buildNodes(layout, nodesData, reorderType) : []);
    }, [layout, reorderType]);

    React.useEffect(() => {
        const layoutHandler = (event) => {
            const nextLayout = (event?.detail || '').toString().toLowerCase();
            if (nextLayout && layouts.includes(nextLayout)) {
                setLayout(nextLayout);
            }
        };
        const reorderHandler = (event) => {
            const nextReorder = (event?.detail || '').toString().toLowerCase();
            if (nextReorder && reorderOptions.includes(nextReorder)) {
                setReorderType(nextReorder);
            }
        };
        window.addEventListener('layoutChange', layoutHandler);
        window.addEventListener('reorderChange', reorderHandler);
        return () => {
            window.removeEventListener('layoutChange', layoutHandler);
            window.removeEventListener('reorderChange', reorderHandler);
        };
    }, []);

    const onNodeClick = React.useCallback((_, node) => {
            const detailsPanel = document.getElementById('details-panel');
            const detailsContent = document.getElementById('details-content');
        if (!detailsPanel || !detailsContent) return;
        
        const data = nodeLookup.get(node.id);
        if (!data) return;
        
        const metrics = data.metrics || {};
        detailsPanel.classList.add('open');
            detailsContent.innerHTML = `
            <div class="details-heading">
                <div class="details-icon">${escapeHtml(data.icon)}</div>
                <div class="details-title">
                    <h3>${escapeHtml(data.name)}</h3>
                    <div class="details-meta">${escapeHtml(data.moduleType)} · ${escapeHtml(data.status)}</div>
                    </div>
                            </div>
            <div class="details-section">
                <h4>Summary</h4>
                <p class="details-path">${escapeHtml(data.filePath)}</p>
                            </div>
            <div class="details-section">
                <h4>Metrics</h4>
                <div class="metric-grid">
                    <div class="metric-item"><span class="metric-item__label">Lines</span><span class="metric-item__value">${formatNumber(metrics.lines_of_code)}</span></div>
                    <div class="metric-item"><span class="metric-item__label">Functions</span><span class="metric-item__value">${formatNumber(metrics.function_count)}</span></div>
                    <div class="metric-item"><span class="metric-item__label">Complexity</span><span class="metric-item__value">${formatNumber(metrics.complexity_score,1)}</span></div>
                    <div class="metric-item"><span class="metric-item__label">Deps</span><span class="metric-item__value">${formatNumber(metrics.dependency_count)}</span></div>
                            </div>
                            </div>
            <div class="details-section">
                <h4>Dependencies</h4>
                <div class="chip-row">${(data.dependencies || []).map((item) => `<span class="chip">${escapeHtml(item)}</span>`).join('') || '<span class="empty-state">None</span>'}</div>
                        </div>
            <div class="details-section">
                <h4>Dependents</h4>
                <div class="chip-row">${(data.dependents || []).map((item) => `<span class="chip">${escapeHtml(item)}</span>`).join('') || '<span class="empty-state">None</span>'}</div>
                    </div>
        `;
    }, []);

    const onPaneClick = React.useCallback(() => {
        const detailsPanel = document.getElementById('details-panel');
        const detailsContent = document.getElementById('details-content');
        if (detailsPanel) detailsPanel.classList.remove('open');
        if (detailsContent) detailsContent.innerHTML = '<p class="details-placeholder">Click on a module to see details</p>';
    }, []);

    const onNodesChange = React.useCallback(
        (changes) => setNodes((nds) => applyNodeChanges(changes, nds)),
        []
    );

    const onEdgesChange = React.useCallback(
        (changes) => setEdges((eds) => applyEdgeChanges(changes, eds)),
        []
    );

    if (!nodesData.length) {
        return e('div', { className: 'empty-architecture' }, 'No modules found');
    }

    return e(ReactFlow, {
        nodes,
        edges,
        nodeTypes,
        onNodesChange,
        onEdgesChange,
        onNodeClick,
        onPaneClick,
        fitView: true,
        defaultEdgeOptions: { type: 'smoothstep', markerEnd: { type: MarkerType.ArrowClosed, width: 20, height: 20 } },
        minZoom: 0.1,
        maxZoom: 1.5,
        proOptions: { hideAttribution: true }
    },
        e(Background, { gap: 32, size: 1, color: '#dce2f2' }),
        e(MiniMap, { nodeColor: (node) => node?.data?.color || '#9ca3af' }),
        e(Controls, null)
    );
};

// Initialize the app when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    // Setup non-React event handlers
    const themeButton = document.getElementById('theme-btn');
    if ((architectureData?.settings?.theme || '').toLowerCase() === 'dark') {
        document.body.classList.add('theme-dark');
    }
    if (themeButton) {
        themeButton.addEventListener('click', () => {
            document.body.classList.toggle('theme-dark');
        });
    }

    const refreshButton = document.getElementById('refresh-btn');
    if (refreshButton) {
        refreshButton.addEventListener('click', () => window.location.reload());
    }

    const closeButton = document.getElementById('close-details');
    if (closeButton) {
        closeButton.addEventListener('click', () => {
            const detailsPanel = document.getElementById('details-panel');
            const detailsContent = document.getElementById('details-content');
            if (detailsPanel) detailsPanel.classList.remove('open');
            if (detailsContent) detailsContent.innerHTML = '<p class="details-placeholder">Click on a module to see details</p>';
        });
    }

           // Layout button handlers
           const layoutButtons = {
               'layout-grid': 'grid',
               'layout-circular': 'circular', 
               'layout-hierarchical': 'hierarchical'
           };
           
           Object.entries(layoutButtons).forEach(([buttonId, layout]) => {
               const button = document.getElementById(buttonId);
               if (button) {
                   button.addEventListener('click', () => {
                       // Remove active class from all layout buttons
                       Object.keys(layoutButtons).forEach(id => {
                           const btn = document.getElementById(id);
                           if (btn) btn.classList.remove('active');
                       });
                       // Add active class to clicked button
                       button.classList.add('active');
                       // Trigger layout change
                       window.dispatchEvent(new CustomEvent('layoutChange', { detail: layout }));
                   });
               }
           });

           // Reorder button handlers
           const reorderButtons = {
               'reorder-hierarchical': 'hierarchical',
               'reorder-grouped': 'grouped-by-type',
               'reorder-dependency': 'dependency-driven',
               'reorder-alphabetical': 'alphabetical'
           };
           
           Object.entries(reorderButtons).forEach(([buttonId, reorderType]) => {
               const button = document.getElementById(buttonId);
               if (button) {
                   button.addEventListener('click', () => {
                       // Remove active class from all reorder buttons
                       Object.keys(reorderButtons).forEach(id => {
                           const btn = document.getElementById(id);
                           if (btn) btn.classList.remove('active');
                       });
                       // Add active class to clicked button
                       button.classList.add('active');
                       // Trigger reorder change
                       window.dispatchEvent(new CustomEvent('reorderChange', { detail: reorderType }));
                   });
               }
           });

           // Legend toggle handler
           const legendToggle = document.getElementById('legend-toggle');
           const legend = document.querySelector('.legend');
           if (legendToggle && legend) {
               legendToggle.addEventListener('click', () => {
                   legend.classList.toggle('visible');
                   legendToggle.textContent = legend.classList.contains('visible') ? '📋 Hide Legend' : '📋 Legend';
               });
           }

    document.addEventListener('keydown', (event) => {
        if (event.key === 'Escape') {
            const detailsPanel = document.getElementById('details-panel');
            const detailsContent = document.getElementById('details-content');
            if (detailsPanel) detailsPanel.classList.remove('open');
            if (detailsContent) detailsContent.innerHTML = '<p class="details-placeholder">Click on a module to see details</p>';
        }
    });

    // Initialize React app
    const rootElement = document.getElementById('react-flow-root');
    if (rootElement) {
        const root = createRoot(rootElement);
        root.render(e(ReactFlowProvider, null, e(FlowApp, null)));
    } else {
        console.error('React Flow root element not found');
    }
});
"#;
        Ok(template.replace("__ARCHITECTURE_DATA__", &serialized))
    }
}
