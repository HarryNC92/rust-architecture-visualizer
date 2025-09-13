# 🚀 Quick Start Guide

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-architecture-visualizer
cd rust-architecture-visualizer

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Basic Usage

### 1. Scan a Project

```bash
# Scan current directory
rust-arch-viz scan .

# Scan specific project
rust-arch-viz scan /path/to/rust/project

# Save results to file
rust-arch-viz scan . --output architecture.json
```

### 2. Start Web Server

```bash
# Start server on default port (8080)
rust-arch-viz serve

# Start on custom port
rust-arch-viz serve --port 3000

# Start with specific project
rust-arch-viz serve --project ./my-rust-project --port 8080
```

### 3. Watch Mode (Auto-refresh)

```bash
# Watch project for changes
rust-arch-viz watch --project ./my-rust-project

# Watch with custom port
rust-arch-viz watch --project . --port 3000
```

## Configuration

Create a `rust-arch-viz.toml` file in your project root:

```toml
[project]
name = "My Awesome Project"
description = "A fantastic Rust project"

[scanning]
include_tests = true
exclude_patterns = ["target/**", "**/.*"]
scan_interval = 30

[visualization]
theme = "dark"
layout = "force_directed"
show_metrics = true
show_dependencies = true

[server]
port = 8080
host = "127.0.0.1"
enable_websocket = true
```

## Features

### 🎯 **Auto-Discovery**

- Automatically finds Rust projects
- Scans `Cargo.toml` for project metadata
- Detects module structure and dependencies

### 🌐 **Beautiful Web Interface**

- Interactive, responsive visualization
- Real-time updates via WebSocket
- Multiple themes and layouts
- Mobile-friendly design

### 🔄 **Real-time Updates**

- Live architecture changes
- File watching in watch mode
- WebSocket connections for instant updates

### 🎯 **Dependency Flow**

- SVG arrows showing module relationships
- Circular dependency detection
- Dependency strength visualization
- Interactive dependency exploration

### ⚙️ **Highly Configurable**

- Project-specific settings
- File pattern filtering
- Theme and layout customization
- Performance tuning options

### 📊 **Rich Metrics**

- Code complexity analysis
- Lines of code counting
- Function/struct/enum/trait counts
- Maintainability index
- Dependency density

## Examples

### Basic Project Analysis

```rust
use rust_architecture_visualizer::{ArchitectureScanner, default_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let scanner = ArchitectureScanner::new(".", default_config());
    let architecture = scanner.scan().await?;

    println!("Found {} modules", architecture.total_modules);
    println!("Total lines: {}", architecture.total_lines);
    println!("Dependencies: {}", architecture.edges.len());

    Ok(())
}
```

### Custom Configuration

```rust
use rust_architecture_visualizer::{ProjectConfig, Theme, LayoutType};

let config = ProjectConfig {
    project: ProjectSettings {
        name: Some("My Project".to_string()),
        description: Some("A great Rust project".to_string()),
        ..Default::default()
    },
    scanning: ScanningSettings {
        include_tests: true,
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

### Web Server Integration

```rust
use rust_architecture_visualizer::{WebServer, ArchitectureVisualizer, ArchitectureScanner, default_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let scanner = ArchitectureScanner::new(".", default_config());
    let visualizer = ArchitectureVisualizer::new(scanner);
    let server = WebServer::new(visualizer);

    server.serve("127.0.0.1", 8080).await?;
    Ok(())
}
```

## API Endpoints

When running the web server:

- `GET /` - Main visualization page
- `GET /api/architecture` - Architecture data (JSON)
- `POST /api/refresh` - Refresh architecture data
- `GET /api/config` - Current configuration
- `GET /api/metrics` - Architecture metrics
- `GET /health` - Health check
- `WS /ws` - WebSocket for real-time updates

## Command Line Options

```bash
# Scan command
rust-arch-viz scan [OPTIONS] <PROJECT>
    --output <OUTPUT>     Output file for architecture data
    --config <CONFIG>     Configuration file path
    --help               Print help information

# Serve command
rust-arch-viz serve [OPTIONS]
    --port <PORT>        Port to run server on (default: 8080)
    --host <HOST>        Host to bind to (default: 127.0.0.1)
    --project <PROJECT>  Project directory to scan (default: .)
    --config <CONFIG>    Configuration file path
    --help              Print help information

# Watch command
rust-arch-viz watch [OPTIONS] <PROJECT>
    --port <PORT>        Port to run server on (default: 8080)
    --config <CONFIG>    Configuration file path
    --help              Print help information
```

## Docker Usage

```dockerfile
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM nginx:alpine
COPY --from=builder /app/target/release/rust-arch-viz /usr/local/bin/
COPY nginx.conf /etc/nginx/nginx.conf
EXPOSE 80
CMD ["rust-arch-viz", "serve", "--host", "0.0.0.0", "--port", "80"]
```

## Integration with CI/CD

```yaml
# GitHub Actions example
name: Architecture Analysis
on: [push, pull_request]

jobs:
  architecture:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Rust Architecture Visualizer
        run: cargo install --path .
      - name: Generate Architecture Report
        run: rust-arch-viz scan . --output architecture.json
      - name: Upload Architecture Report
        uses: actions/upload-artifact@v2
        with:
          name: architecture-report
          path: architecture.json
```

## Troubleshooting

### Common Issues

1. **Port already in use**: Change the port with `--port` option
2. **Permission denied**: Run with appropriate permissions
3. **Configuration not found**: Create `rust-arch-viz.toml` file
4. **Scanning too slow**: Increase `scan_interval` in config
5. **Memory usage high**: Reduce `max_file_size` in config

### Debug Mode

```bash
RUST_LOG=debug rust-arch-viz serve
```

### Performance Tips

1. **Exclude unnecessary files**: Use `exclude_patterns` in config
2. **Increase scan interval**: For large projects, use 60+ seconds
3. **Limit file size**: Set appropriate `max_file_size`
4. **Use appropriate layout**: Grid for small projects, Force Directed for complex ones

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Support

- 📖 Documentation: [docs/](docs/)
- 🐛 Issues: [GitHub Issues](https://github.com/yourusername/rust-architecture-visualizer/issues)
- 💬 Discussions: [GitHub Discussions](https://github.com/yourusername/rust-architecture-visualizer/discussions)
- 📧 Email: support@example.com

---

**Made with ❤️ for the Rust community**
