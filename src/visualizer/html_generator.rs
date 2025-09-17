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
                <div class="architecture-canvas" id="react-flow-root">
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
body.theme-dark .btn-secondary{background:rgba(30,41,59,.7);border-color:rgba(148,163,184,.4);color:#e2e8f0;}
.btn-close{background:var(--danger);color:#fff;width:2rem;height:2rem;border-radius:50%;display:flex;align-items:center;justify-content:center;}
.stats{display:grid;grid-template-columns:repeat(auto-fit,minmax(200px,1fr));gap:1.4rem;padding:1.8rem;background:rgba(248,250,252,.9);}
body.theme-dark .stats{background:rgba(15,23,42,.72);}
.stat-card{background:#fff;border-radius:16px;padding:1.4rem;text-align:center;box-shadow:0 10px 24px rgba(15,23,42,.12);}
body.theme-dark .stat-card{background:rgba(30,41,59,.92);color:#e2e8f0;}
.stat-number{font-size:2.2rem;font-weight:700;color:#667eea;}
.stat-label{text-transform:uppercase;font-size:.78rem;letter-spacing:.08em;color:#64748b;}
.visualization-container{display:grid;grid-template-columns:minmax(0,1fr)320px;min-height:600px;}
@media(max-width:1000px){.visualization-container{grid-template-columns:1fr;}}
.visualization-panel{position:relative;padding:1.5rem;background:linear-gradient(135deg,rgba(102,126,234,.08),rgba(118,75,162,.08));}
.legend{position:absolute;top:1.3rem;right:1.3rem;background:#fff;border-radius:12px;padding:1rem 1.1rem;box-shadow:0 12px 24px rgba(15,23,42,.12);z-index:10;}
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
.rf-module-card{padding:.9rem 1rem;display:flex;flex-direction:column;gap:.75rem;}
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
import * as React from 'https://esm.sh/react@18.2.0?dev';
import * as ReactDOMClient from 'https://esm.sh/react-dom@18.2.0/client?dev';
import ReactFlow, { Background, Controls, MiniMap, MarkerType, ReactFlowProvider, applyEdgeChanges, applyNodeChanges } from 'https://esm.sh/reactflow@11.7.4?deps=react@18.2.0,react-dom@18.2.0&dev';

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

const layouts = ['grid', 'circular'];
let currentLayoutIndex = Math.max(layouts.indexOf((architectureData.layout || 'grid').toLowerCase()), 0);

// Utility functions
const escapeHtml = (value) => value === null || value === undefined ? '' : String(value).replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;').replace(/'/g,'&#39;');
const formatNumber = (value, digits = 0) => value === null || value === undefined ? '—' : Number(value).toLocaleString(undefined, { maximumFractionDigits: digits });

const computePositions = (layout, nodes) => {
    const positions = new Map();
    const total = nodes.length || 1;
    if (layout === 'circular' && total > 1) {
        const radius = 220 + total * 12;
        const cx = radius + 180;
        const cy = radius * 0.55 + 150;
        nodes.forEach((node, index) => {
            const angle = (Math.PI * 2 * index) / total;
            positions.set(node.id, { x: cx + Math.cos(angle) * radius, y: cy + Math.sin(angle) * radius * 0.7 });
        });
    }
    if (!positions.size) {
        const columns = Math.ceil(Math.sqrt(total));
        nodes.forEach((node, index) => {
            const column = index % columns;
            const row = Math.floor(index / columns);
            positions.set(node.id, { x: 160 + column * 240, y: 140 + row * 200 });
        });
    }
    return positions;
};

const buildNodes = (layout, nodes) => {
    const positions = computePositions(layout, nodes);
    return nodes.map((node) => ({
        id: node.id,
        type: 'module',
        position: positions.get(node.id) || { x: 0, y: 0 },
        data: { ...node },
        className: '',
        sourcePosition: 'right',
        targetPosition: 'left'
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

        return {
            ...edge,
            id: edge?.id ?? `edge-${source}-${target}-${index}`,
            source,
            target,
            style: { ...(edge.style || {}) },
            labelStyle: { fill: '#1f2937', fontSize: 11, fontWeight: 600 },
            labelBgPadding: [6, 3],
            labelBgBorderRadius: 6,
            labelBgStyle: { fill: 'rgba(255,255,255,0.9)' }
        };
    })
    .filter(Boolean);

// React components
const e = React.createElement;

const ModuleNode = ({ data }) => {
    const metrics = data?.metrics || {};
    const showMetrics = architectureData?.settings?.showMetrics === true;
    
    return e('div', { className: 'rf-module-card' },
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
    const [nodes, setNodes] = React.useState(() =>
        nodesData.length ? buildNodes(layouts[currentLayoutIndex] || 'grid', nodesData) : []
    );
    const [edges, setEdges] = React.useState(() =>
        edgesData.length ? buildEdges(edgesData) : []
    );
    const nodeTypes = React.useMemo(() => ({ module: ModuleNode }), []);

    React.useEffect(() => {
        setNodes(nodesData.length ? buildNodes(layout, nodesData) : []);
    }, [layout]);

    React.useEffect(() => {
        const handler = (event) => {
            const nextLayout = (event?.detail || '').toString().toLowerCase();
            if (nextLayout && layouts.includes(nextLayout)) {
                setLayout(nextLayout);
            }
        };
        window.addEventListener('layoutChange', handler);
        return () => window.removeEventListener('layoutChange', handler);
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

    // Layout button handler
    const layoutButton = document.getElementById('layout-btn');
    if (layoutButton) {
        const updateLabel = () => {
            const name = layouts[currentLayoutIndex];
            layoutButton.textContent = `📐 Layout (${name.charAt(0).toUpperCase() + name.slice(1)})`;
        };
        updateLabel();
        layoutButton.addEventListener('click', () => {
            currentLayoutIndex = (currentLayoutIndex + 1) % layouts.length;
            updateLabel();
            // Trigger a re-render by dispatching a custom event
            window.dispatchEvent(new CustomEvent('layoutChange', { detail: layouts[currentLayoutIndex] }));
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
