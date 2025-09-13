use anyhow::Result;
use rust_architecture_visualizer::{
    ArchitectureScanner, ArchitectureVisualizer, ProjectConfig, VisualizationSettings, 
    ScanningSettings, ServerSettings, Theme, LayoutType
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    println!("🚀 Rust Architecture Visualizer - Custom Configuration Example");

    // Create custom configuration
    let config = ProjectConfig {
        project: rust_architecture_visualizer::ProjectSettings {
            name: Some("My Awesome Rust Project".to_string()),
            description: Some("A fantastic Rust project with custom visualization".to_string()),
            version: Some("1.0.0".to_string()),
            authors: vec!["Developer".to_string()],
            repository: Some("https://github.com/developer/my-project".to_string()),
        },
        scanning: ScanningSettings {
            include_tests: true,
            include_examples: true,
            include_benches: false,
            include_docs: false,
            exclude_patterns: vec![
                "target/**".to_string(),
                "**/target/**".to_string(),
                "**/.git/**".to_string(),
                "**/node_modules/**".to_string(),
                "**/.*".to_string(),
                "**/test_*".to_string(), // Exclude test files
            ],
            include_patterns: vec![
                "**/*.rs".to_string(),
                "**/src/**".to_string(),
            ],
            scan_interval: 10, // Scan every 10 seconds
            max_file_size: Some(5 * 1024 * 1024), // 5MB max file size
            follow_symlinks: false,
            ignore_gitignore: true,
        },
        visualization: VisualizationSettings {
            theme: Theme::Dark,
            layout: LayoutType::ForceDirected,
            show_metrics: true,
            show_dependencies: true,
            show_errors: true,
            show_warnings: true,
            group_by_type: true,
            show_file_paths: true,
            show_documentation: true,
            filter_complexity: Some(5.0), // Only show modules with complexity > 5
            filter_type: None,
            auto_refresh: true,
            refresh_interval: 30,
        },
        server: ServerSettings {
            port: 3000,
            host: "0.0.0.0".to_string(), // Listen on all interfaces
            cors_origins: vec!["*".to_string()],
            enable_websocket: true,
            enable_compression: true,
            max_request_size: Some(10 * 1024 * 1024), // 10MB
            timeout: Some(60), // 60 seconds timeout
        },
    };

    // Create scanner with custom config
    let scanner = ArchitectureScanner::new(".", config);
    
    // Scan the project
    println!("📊 Scanning project with custom configuration...");
    let architecture = scanner.scan().await?;
    
    println!("✅ Found {} modules", architecture.total_modules);
    println!("📈 Total lines of code: {}", architecture.total_lines);
    println!("🔗 Dependencies: {}", architecture.edges.len());
    println!("⚠️  Circular dependencies: {}", architecture.circular_dependencies.len());
    
    // Print architecture metrics
    println!("\n📊 Architecture Metrics:");
    println!("  Total functions: {}", architecture.metrics.total_functions);
    println!("  Total structs: {}", architecture.metrics.total_structs);
    println!("  Total enums: {}", architecture.metrics.total_enums);
    println!("  Total traits: {}", architecture.metrics.total_traits);
    println!("  Max complexity: {:.1}", architecture.metrics.max_complexity);
    println!("  Min complexity: {:.1}", architecture.metrics.min_complexity);
    println!("  Dependency density: {:.2}", architecture.metrics.dependency_density);
    println!("  Modularity score: {:.2}", architecture.metrics.modularity_score);
    println!("  Maintainability index: {:.2}", architecture.metrics.maintainability_index);
    
    // Create visualizer
    let visualizer = ArchitectureVisualizer::new(scanner);
    
    // Generate HTML with custom settings
    println!("🎨 Generating visualization with custom settings...");
    let html = visualizer.generate_html(&architecture)?;
    
    // Save to file
    std::fs::write("custom_architecture.html", html)?;
    println!("💾 Custom visualization saved to custom_architecture.html");
    
    // Save configuration to file
    let config_path = "rust-arch-viz.toml";
    visualizer.get_config().save_to_file(config_path)?;
    println!("💾 Configuration saved to {}", config_path);
    
    Ok(())
}
