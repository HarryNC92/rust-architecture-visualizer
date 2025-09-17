use clap::{Parser, Subcommand};
use rust_architecture_visualizer::{
    config::ProjectConfig,
    scanner::ArchitectureScanner,
    web::WebServer,
    visualizer::ArchitectureVisualizer,
};
use std::path::PathBuf;
use tracing::{info, error};

#[derive(Parser)]
#[command(name = "rust-arch-viz")]
#[command(about = "A beautiful, real-time architecture visualizer for Rust projects")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a Rust project and generate architecture data
    Scan {
        /// Path to the Rust project directory
        #[arg(short, long, default_value = ".")]
        project: PathBuf,
        
        /// Output file for architecture data (JSON)
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Configuration file path
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    
    /// Start the web server for interactive visualization
    Serve {
        /// Port to run the server on
        #[arg(long, default_value = "8000")]
        port: u16,
        
        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        
        /// Path to the Rust project directory
        #[arg(short, long, default_value = ".")]
        project: PathBuf,
        
        /// Configuration file path
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    
    /// Watch a project for changes and auto-refresh
    Watch {
        /// Path to the Rust project directory
        #[arg(short, long, default_value = ".")]
        project: PathBuf,
        
        /// Port to run the server on
        #[arg(long, default_value = "8000")]
        port: u16,
        
        /// Configuration file path
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { project, output, config } => {
            info!("Scanning project at: {:?}", project);
            
            let config = if let Some(config_path) = config {
                ProjectConfig::from_file(&config_path)?
            } else {
                ProjectConfig::from_project_dir(&project)?
            };
            
            let scanner = ArchitectureScanner::new(&project, config);
            let architecture = scanner.scan_async().await?;
            
            if let Some(output_path) = output {
                std::fs::write(&output_path, serde_json::to_string_pretty(&architecture)?)?;
                info!("Architecture data saved to: {:?}", output_path);
            } else {
                println!("{}", serde_json::to_string_pretty(&architecture)?);
            }
        }
        
        Commands::Serve { port, host, project, config } => {
            info!("Starting web server on {}:{}", host, port);
            info!("Project directory: {:?}", project);
            
            let config = if let Some(config_path) = config {
                ProjectConfig::from_file(&config_path)?
            } else {
                ProjectConfig::from_project_dir(&project)?
            };
            
            let scanner = ArchitectureScanner::new(&project, config);
            let visualizer = ArchitectureVisualizer::new(scanner);
            let server = WebServer::new(visualizer);
            
            server.serve(&host, port).await?;
        }
        
        Commands::Watch { project, port, config } => {
            info!("Starting watch mode for project: {:?}", project);
            
            let config = if let Some(config_path) = config {
                ProjectConfig::from_file(&config_path)?
            } else {
                ProjectConfig::from_project_dir(&project)?
            };
            
            let scanner = ArchitectureScanner::new(&project, config);
            let visualizer = ArchitectureVisualizer::new(scanner);
            let server = WebServer::new(visualizer);
            
            // Enable watch mode and serve
            server.watch_mode(true).serve("127.0.0.1", port).await?;
        }
    }

    Ok(())
}
