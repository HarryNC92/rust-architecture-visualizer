use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Json},
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    web::WebState,
    types::{ArchitectureMap, VisualizationSettings},
};

/// Main index page handler
pub async fn index_handler(State(state): State<WebState>) -> Result<Html<String>, StatusCode> {
    let visualizer = state.visualizer.read().await;
    let architecture = visualizer.get_architecture().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let html = visualizer.generate_html(&architecture)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Html(html))
}

/// Architecture data API handler
pub async fn architecture_handler(State(state): State<WebState>) -> Result<Json<ArchitectureMap>, StatusCode> {
    let visualizer = state.visualizer.read().await;
    let architecture = visualizer.get_architecture().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(architecture))
}

/// Refresh architecture data handler
pub async fn refresh_handler(State(state): State<WebState>) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut visualizer = state.visualizer.write().await;
    let result = visualizer.refresh().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(json!({
        "success": true,
        "message": "Architecture refreshed successfully",
        "timestamp": result.last_scan
    })))
}

/// Configuration handler
pub async fn config_handler(State(state): State<WebState>) -> Result<Json<serde_json::Value>, StatusCode> {
    let visualizer = state.visualizer.read().await;
    let config = visualizer.get_config();
    
    Ok(Json(json!({
        "project": {
            "name": config.project.name,
            "description": config.project.description,
            "version": config.project.version
        },
        "visualization": {
            "theme": config.visualization.theme,
            "layout": config.visualization.layout,
            "show_metrics": config.visualization.show_metrics,
            "show_dependencies": config.visualization.show_dependencies,
            "auto_refresh": config.visualization.auto_refresh,
            "refresh_interval": config.visualization.refresh_interval
        },
        "server": {
            "port": config.server.port,
            "host": config.server.host,
            "watch_mode": state.watch_mode
        }
    })))
}

/// Metrics handler
pub async fn metrics_handler(State(state): State<WebState>) -> Result<Json<serde_json::Value>, StatusCode> {
    let visualizer = state.visualizer.read().await;
    let architecture = visualizer.get_architecture().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(json!({
        "total_modules": architecture.total_modules,
        "total_lines": architecture.total_lines,
        "average_complexity": architecture.average_complexity,
        "total_dependencies": architecture.edges.len(),
        "circular_dependencies": architecture.circular_dependencies.len(),
        "metrics": architecture.metrics,
        "last_scan": architecture.last_scan
    })))
}

/// Static file handler (for serving assets)
pub async fn static_handler() -> Result<Html<&'static str>, StatusCode> {
    // For now, return a simple message
    // In a real implementation, you'd serve static files
    Ok(Html("<h1>Static files not implemented yet</h1>"))
}

/// Health check handler
pub async fn health_handler() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "version": env!("CARGO_PKG_VERSION")
    })))
}
