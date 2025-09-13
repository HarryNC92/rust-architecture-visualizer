use anyhow::Result;
use rust_architecture_visualizer::{ArchitectureScanner, ArchitectureVisualizer, default_config};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    println!("🚀 Rust Architecture Visualizer - Basic Usage Example");

    // Create a scanner for the current directory
    let config = default_config();
    let scanner = ArchitectureScanner::new(".", config);
    
    // Scan the project
    println!("📊 Scanning project...");
    let architecture = scanner.scan().await?;
    
    println!("✅ Found {} modules", architecture.total_modules);
    println!("📈 Total lines of code: {}", architecture.total_lines);
    println!("🔗 Dependencies: {}", architecture.edges.len());
    println!("⚠️  Circular dependencies: {}", architecture.circular_dependencies.len());
    
    // Create visualizer
    let visualizer = ArchitectureVisualizer::new(scanner);
    
    // Generate HTML
    println!("🎨 Generating visualization...");
    let html = visualizer.generate_html(&architecture)?;
    
    // Save to file
    std::fs::write("architecture.html", html)?;
    println!("💾 Visualization saved to architecture.html");
    
    // Print some module details
    println!("\n📋 Module Details:");
    for (i, node) in architecture.nodes.values().enumerate() {
        if i >= 5 { // Show only first 5 modules
            println!("... and {} more modules", architecture.nodes.len() - 5);
            break;
        }
        
        println!("  {} - {} ({:?}) - {} lines, {:.1} complexity",
            node.name,
            node.module_type.icon(),
            node.module_type,
            node.metrics.lines_of_code,
            node.metrics.complexity_score
        );
    }
    
    Ok(())
}
