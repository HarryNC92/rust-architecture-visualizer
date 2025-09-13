# API Reference

## Overview

The Rust Architecture Visualizer provides a comprehensive API for analyzing and visualizing Rust project architectures. This document covers all public APIs and their usage.

## Core Types

### ArchitectureMap

The main data structure containing the complete architecture information.

```rust
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
```

### ArchitectureNode

Represents a single module in the architecture.

```rust
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
```

## Scanner API

### ArchitectureScanner

The main scanner for analyzing Rust projects.

```rust
impl ArchitectureScanner {
    pub fn new<P: AsRef<Path>>(project_path: P, config: ProjectConfig) -> Self;
    pub async fn scan(&self) -> Result<ArchitectureMap>;
}
```

**Example:**

```rust
use rust_architecture_visualizer::{ArchitectureScanner, default_config};

let scanner = ArchitectureScanner::new("./my-project", default_config());
let architecture = scanner.scan().await?;
```

## Visualizer API

### ArchitectureVisualizer

Generates HTML visualizations from architecture data.

```rust
impl ArchitectureVisualizer {
    pub fn new(scanner: ArchitectureScanner) -> Self;
    pub async fn get_architecture(&self) -> Result<ArchitectureMap>;
    pub async fn refresh(&mut self) -> Result<ArchitectureMap>;
    pub fn get_config(&self) -> &ProjectConfig;
    pub fn generate_html(&self, architecture: &ArchitectureMap) -> Result<String>;
}
```

**Example:**

```rust
use rust_architecture_visualizer::{ArchitectureVisualizer, ArchitectureScanner, default_config};

let scanner = ArchitectureScanner::new(".", default_config());
let visualizer = ArchitectureVisualizer::new(scanner);
let architecture = visualizer.get_architecture().await?;
let html = visualizer.generate_html(&architecture)?;
```

## Web Server API

### WebServer

Starts a web server for interactive visualization.

```rust
impl WebServer {
    pub fn new(visualizer: ArchitectureVisualizer) -> Self;
    pub fn watch_mode(self, enabled: bool) -> Self;
    pub async fn serve(self, host: &str, port: u16) -> Result<()>;
}
```

**Example:**

```rust
use rust_architecture_visualizer::{WebServer, ArchitectureVisualizer, ArchitectureScanner, default_config};

let scanner = ArchitectureScanner::new(".", default_config());
let visualizer = ArchitectureVisualizer::new(scanner);
let server = WebServer::new(visualizer);
server.serve("127.0.0.1", 8080).await?;
```

## Configuration API

### ProjectConfig

Configuration for project scanning and visualization.

```rust
pub struct ProjectConfig {
    pub project: ProjectSettings,
    pub scanning: ScanningSettings,
    pub visualization: VisualizationSettings,
    pub server: ServerSettings,
}
```

**Example:**

```rust
use rust_architecture_visualizer::{ProjectConfig, Theme, LayoutType};

let config = ProjectConfig {
    project: ProjectSettings {
        name: Some("My Project".to_string()),
        description: Some("A great Rust project".to_string()),
        version: Some("1.0.0".to_string()),
        authors: vec!["Developer".to_string()],
        repository: Some("https://github.com/developer/project".to_string()),
    },
    scanning: ScanningSettings {
        include_tests: true,
        include_examples: false,
        exclude_patterns: vec!["target/**".to_string()],
        scan_interval: 30,
        ..Default::default()
    },
    visualization: VisualizationSettings {
        theme: Theme::Dark,
        layout: LayoutType::ForceDirected,
        show_metrics: true,
        show_dependencies: true,
        ..Default::default()
    },
    server: ServerSettings {
        port: 8080,
        host: "127.0.0.1".to_string(),
        enable_websocket: true,
        ..Default::default()
    },
};
```

## HTTP API Endpoints

When running the web server, the following endpoints are available:

### GET /

Returns the main visualization page.

### GET /api/architecture

Returns the current architecture data as JSON.

**Response:**

```json
{
  "nodes": { ... },
  "edges": [ ... ],
  "last_scan": "2024-01-01T00:00:00Z",
  "total_modules": 10,
  "total_lines": 5000,
  "average_complexity": 3.2,
  "circular_dependencies": [],
  "metrics": { ... }
}
```

### POST /api/refresh

Triggers a refresh of the architecture data.

**Response:**

```json
{
  "success": true,
  "message": "Architecture refreshed successfully",
  "timestamp": "2024-01-01T00:00:00Z"
}
```

### GET /api/config

Returns the current configuration.

**Response:**

```json
{
  "project": {
    "name": "My Project",
    "description": "A great Rust project",
    "version": "1.0.0"
  },
  "visualization": {
    "theme": "dark",
    "layout": "force_directed",
    "show_metrics": true,
    "show_dependencies": true
  },
  "server": {
    "port": 8080,
    "host": "127.0.0.1",
    "watch_mode": false
  }
}
```

### GET /api/metrics

Returns architecture metrics.

**Response:**

```json
{
  "total_modules": 10,
  "total_lines": 5000,
  "average_complexity": 3.2,
  "total_dependencies": 15,
  "circular_dependencies": 0,
  "metrics": {
    "total_functions": 50,
    "total_structs": 20,
    "total_enums": 5,
    "total_traits": 10,
    "max_complexity": 8.5,
    "min_complexity": 1.0,
    "dependency_density": 0.15,
    "modularity_score": 0.8,
    "maintainability_index": 0.7
  },
  "last_scan": "2024-01-01T00:00:00Z"
}
```

### GET /health

Health check endpoint.

**Response:**

```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T00:00:00Z",
  "version": "0.1.0"
}
```

## WebSocket API

### /ws

General WebSocket connection for real-time updates.

**Message Types:**

#### Client → Server

```json
{
  "type": "ping"
}
```

```json
{
  "type": "refresh"
}
```

#### Server → Client

```json
{
  "type": "pong",
  "timestamp": "2024-01-01T00:00:00Z"
}
```

```json
{
  "type": "architecture",
  "data": { ... }
}
```

```json
{
  "type": "error",
  "message": "Error description"
}
```

### /ws/architecture

Architecture-specific WebSocket connection with enhanced features.

Same message types as `/ws` but optimized for architecture data.

## Error Handling

All APIs return `Result<T, anyhow::Error>` for error handling. Common error types:

- **FileNotFound**: Project directory or files not found
- **ParseError**: Invalid configuration or project structure
- **NetworkError**: Web server or WebSocket connection issues
- **ScanError**: Architecture scanning failed

**Example:**

```rust
use anyhow::Result;

async fn scan_project() -> Result<()> {
    let scanner = ArchitectureScanner::new(".", default_config());
    let architecture = scanner.scan().await?; // Returns Result<ArchitectureMap, anyhow::Error>
    Ok(())
}
```

## Examples

See the `examples/` directory for complete usage examples:

- `basic_usage.rs` - Basic scanning and visualization
- `custom_config.rs` - Custom configuration example
- `web_server.rs` - Web server setup
- `watch_mode.rs` - File watching and auto-refresh
