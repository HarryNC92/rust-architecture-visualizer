# 🏗️ Rust Architecture Visualizer

A beautiful, real-time architecture visualizer for Rust projects that automatically discovers and visualizes your codebase structure, dependencies, and relationships.

![Architecture Visualizer](https://img.shields.io/badge/rust-architecture--visualizer-blue?style=for-the-badge&logo=rust)
![Version](https://img.shields.io/badge/version-0.1.0-green?style=for-the-badge)
![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)

## ✨ Features

- 🔍 **Auto-Discovery**: Automatically finds and scans Rust projects
- 🌐 **Beautiful Web Interface**: Interactive, responsive architecture visualization
- 🔄 **Real-time Updates**: Live architecture changes via WebSocket
- 🎯 **Dependency Flow**: SVG arrows showing module relationships
- ⚙️ **Configurable**: Project-specific settings and filters
- 🚀 **Fast**: Optimized scanning and caching
- 📊 **Metrics**: Code complexity, test coverage, and performance stats
- 🎨 **Customizable**: Themes and layout options

## 🚀 Quick Start

### Installation

```bash
# From crates.io (when published)
cargo install rust-architecture-visualizer

# From source
git clone https://github.com/harrync/rust-architecture-visualizer
cd rust-architecture-visualizer
cargo build --release
```

### Basic Usage

```bash
# Scan current directory
rust-arch-viz scan .

# Start web server
rust-arch-viz serve --port 8000

# Watch mode with auto-refresh
rust-arch-viz watch --project ./my-rust-project

# current project:
cargo run -- watch --project /Users/harrync/repos/titan-agent-backend
```

### Web Interface

Open your browser to `http://localhost:8000` to see your architecture visualization!

## 📖 Documentation

- [API Reference](docs/API.md)
- [Configuration Guide](docs/CONFIGURATION.md)
- [Examples](examples/)
- [Contributing](CONTRIBUTING.md)

## 🎯 Use Cases

- **Code Review**: Understand project structure before diving in
- **Onboarding**: Help new team members understand the codebase
- **Refactoring**: Visualize dependencies before making changes
- **Documentation**: Generate living architecture diagrams
- **Analysis**: Identify complex modules and dependencies

## 🔧 Configuration

Create a `rust-arch-viz.toml` file in your project root:

```toml
[project]
name = "my-awesome-project"
description = "A fantastic Rust project"

[scanning]
include_tests = true
include_examples = false
exclude_patterns = ["target/**", "**/test_*"]
scan_interval = 30

[visualization]
theme = "dark"
show_metrics = true
group_by_type = true
show_dependencies = true

[server]
port = 8000
host = "127.0.0.1"
```

## 🛠️ Development

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-architecture-visualizer
cd rust-architecture-visualizer

# Install dependencies
cargo build

# Run tests
cargo test

# Run with watch mode
cargo run -- watch --project ./examples/sample-project
```

## 📊 Screenshots

![Architecture Overview](docs/screenshots/overview.png)
![Dependency Flow](docs/screenshots/dependencies.png)
![Module Details](docs/screenshots/module-details.png)

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by the need for better Rust project visualization
- Built with [Axum](https://github.com/tokio-rs/axum) and [Tokio](https://tokio.rs/)
- SVG rendering powered by custom algorithms

---

**Made with ❤️ for the Rust community**
