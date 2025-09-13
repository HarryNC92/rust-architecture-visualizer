# Configuration Guide

## Overview

The Rust Architecture Visualizer uses a flexible configuration system that allows you to customize scanning behavior, visualization settings, and server options.

## Configuration Files

The visualizer looks for configuration files in the following order:

1. `rust-arch-viz.toml`
2. `rust-arch-viz.yaml`
3. `rust-arch-viz.yml`
4. `rust-arch-viz.json`
5. `.rust-arch-viz.toml`
6. `.rust-arch-viz.yaml`
7. `.rust-arch-viz.yml`
8. `.rust-arch-viz.json`

If no configuration file is found, it will try to load settings from `Cargo.toml` and then fall back to defaults.

## Configuration Structure

### Project Settings

```toml
[project]
name = "My Awesome Project"
description = "A fantastic Rust project"
version = "1.0.0"
authors = ["Developer <dev@example.com>"]
repository = "https://github.com/developer/my-project"
```

### Scanning Settings

```toml
[scanning]
# Include/exclude patterns
include_tests = true
include_examples = false
include_benches = false
include_docs = false

# File patterns
exclude_patterns = [
    "target/**",
    "**/target/**",
    "**/.git/**",
    "**/node_modules/**",
    "**/.*",
    "**/test_*"
]
include_patterns = [
    "**/*.rs",
    "**/src/**"
]

# Scanning behavior
scan_interval = 30  # seconds
max_file_size = 10485760  # 10MB in bytes
follow_symlinks = false
ignore_gitignore = true
```

### Visualization Settings

```toml
[visualization]
# Appearance
theme = "dark"  # "light", "dark", "auto"
layout = "force_directed"  # "grid", "force_directed", "hierarchical", "circular"

# Display options
show_metrics = true
show_dependencies = true
show_errors = true
show_warnings = true
group_by_type = true
show_file_paths = true
show_documentation = true

# Filtering
filter_complexity = 5.0  # Only show modules with complexity > 5
filter_type = "Core"     # Only show specific module type

# Auto-refresh
auto_refresh = true
refresh_interval = 30  # seconds
```

### Server Settings

```toml
[server]
port = 8080
host = "127.0.0.1"
cors_origins = ["*"]
enable_websocket = true
enable_compression = true
max_request_size = 10485760  # 10MB in bytes
timeout = 30  # seconds
```

## Configuration Examples

### Basic Configuration

```toml
[project]
name = "My Project"

[scanning]
include_tests = true
exclude_patterns = ["target/**"]

[visualization]
theme = "auto"
show_metrics = true

[server]
port = 3000
```

### Advanced Configuration

```toml
[project]
name = "Complex Rust Project"
description = "A sophisticated Rust application with multiple modules"
version = "2.1.0"
authors = ["Team Lead <lead@company.com>", "Developer <dev@company.com>"]
repository = "https://github.com/company/complex-project"

[scanning]
include_tests = true
include_examples = true
include_benches = true
include_docs = false

exclude_patterns = [
    "target/**",
    "**/target/**",
    "**/.git/**",
    "**/node_modules/**",
    "**/.*",
    "**/test_*",
    "**/benches/**",
    "**/examples/**"
]

include_patterns = [
    "**/*.rs",
    "**/src/**",
    "**/lib/**",
    "**/bin/**"
]

scan_interval = 15
max_file_size = 5242880  # 5MB
follow_symlinks = false
ignore_gitignore = true

[visualization]
theme = "dark"
layout = "force_directed"
show_metrics = true
show_dependencies = true
show_errors = true
show_warnings = true
group_by_type = true
show_file_paths = true
show_documentation = true
filter_complexity = 3.0
filter_type = null
auto_refresh = true
refresh_interval = 15

[server]
port = 8080
host = "0.0.0.0"
cors_origins = [
    "http://localhost:3000",
    "http://localhost:8080",
    "https://mycompany.com"
]
enable_websocket = true
enable_compression = true
max_request_size = 10485760  # 10MB
timeout = 60
```

### Development Configuration

```toml
[project]
name = "Development Project"

[scanning]
include_tests = true
include_examples = true
exclude_patterns = ["target/**", "**/.*"]

scan_interval = 5  # Very frequent scanning
max_file_size = 2097152  # 2MB

[visualization]
theme = "light"
layout = "grid"
show_metrics = true
show_dependencies = true
show_errors = true
show_warnings = true
group_by_type = false
show_file_paths = true
show_documentation = false
auto_refresh = true
refresh_interval = 5

[server]
port = 3000
host = "127.0.0.1"
enable_websocket = true
```

### Production Configuration

```toml
[project]
name = "Production Application"

[scanning]
include_tests = false
include_examples = false
exclude_patterns = [
    "target/**",
    "**/target/**",
    "**/.git/**",
    "**/node_modules/**",
    "**/.*",
    "**/test_*",
    "**/benches/**",
    "**/examples/**",
    "**/docs/**"
]

scan_interval = 300  # 5 minutes
max_file_size = 10485760  # 10MB

[visualization]
theme = "dark"
layout = "hierarchical"
show_metrics = true
show_dependencies = true
show_errors = false
show_warnings = false
group_by_type = true
show_file_paths = false
show_documentation = true
filter_complexity = 5.0
auto_refresh = false

[server]
port = 8080
host = "0.0.0.0"
cors_origins = ["https://mycompany.com"]
enable_websocket = true
enable_compression = true
max_request_size = 52428800  # 50MB
timeout = 120
```

## Environment Variables

You can override configuration values using environment variables:

```bash
# Project settings
export RUST_ARCH_VIZ_PROJECT_NAME="My Project"
export RUST_ARCH_VIZ_PROJECT_DESCRIPTION="A great project"

# Scanning settings
export RUST_ARCH_VIZ_INCLUDE_TESTS=true
export RUST_ARCH_VIZ_SCAN_INTERVAL=30
export RUST_ARCH_VIZ_MAX_FILE_SIZE=10485760

# Visualization settings
export RUST_ARCH_VIZ_THEME=dark
export RUST_ARCH_VIZ_LAYOUT=force_directed
export RUST_ARCH_VIZ_SHOW_METRICS=true

# Server settings
export RUST_ARCH_VIZ_PORT=8080
export RUST_ARCH_VIZ_HOST=0.0.0.0
export RUST_ARCH_VIZ_ENABLE_WEBSOCKET=true
```

## Configuration Validation

The visualizer validates configuration values and provides helpful error messages:

```bash
$ rust-arch-viz serve --config invalid.toml
Error: Invalid configuration file 'invalid.toml':
  - scanning.scan_interval: must be a positive integer
  - visualization.theme: must be one of 'light', 'dark', 'auto'
  - server.port: must be between 1 and 65535
```

## Dynamic Configuration

You can update configuration at runtime through the API:

```bash
# Get current configuration
curl http://localhost:8080/api/config

# Update configuration (if supported)
curl -X POST http://localhost:8080/api/config \
  -H "Content-Type: application/json" \
  -d '{"visualization": {"theme": "dark"}}'
```

## Best Practices

### 1. Use Appropriate Scan Intervals

- **Development**: 5-15 seconds for frequent updates
- **Testing**: 30-60 seconds for balanced performance
- **Production**: 300+ seconds for minimal resource usage

### 2. Configure File Patterns Carefully

```toml
# Good: Specific patterns
exclude_patterns = [
    "target/**",
    "**/target/**",
    "**/.git/**",
    "**/node_modules/**"
]

# Avoid: Too broad patterns
exclude_patterns = ["**/*"]  # This would exclude everything!
```

### 3. Set Appropriate File Size Limits

```toml
# For small projects
max_file_size = 1048576  # 1MB

# For large projects
max_file_size = 10485760  # 10MB

# For enterprise projects
max_file_size = 52428800  # 50MB
```

### 4. Choose the Right Layout

- **Grid**: Good for small projects with few modules
- **Force Directed**: Good for medium projects with complex dependencies
- **Hierarchical**: Good for large projects with clear hierarchy
- **Circular**: Good for projects with circular dependencies

### 5. Optimize for Your Use Case

```toml
# For code review
[visualization]
show_metrics = true
show_dependencies = true
show_errors = true
show_warnings = true
group_by_type = true

# For presentations
[visualization]
show_metrics = false
show_dependencies = true
show_errors = false
show_warnings = false
group_by_type = true
theme = "dark"
```

## Troubleshooting

### Common Issues

1. **Configuration not loading**: Check file path and format
2. **Scanning too slow**: Increase `scan_interval` or reduce `max_file_size`
3. **Memory usage high**: Reduce `max_file_size` or exclude more patterns
4. **WebSocket not working**: Check `enable_websocket` and CORS settings
5. **Port already in use**: Change `port` in server settings

### Debug Mode

Enable debug logging to troubleshoot issues:

```bash
RUST_LOG=debug rust-arch-viz serve
```

### Configuration Validation

Validate your configuration:

```bash
rust-arch-viz scan --config your-config.toml --validate
```
