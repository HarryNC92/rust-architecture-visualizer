use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{CorsLayer, Any},
    compression::CompressionLayer,
    trace::TraceLayer,
};
use tracing::info;

use crate::{
    web::{handlers, websocket, WebState},
    visualizer::ArchitectureVisualizer,
};

/// Web server for the architecture visualizer
pub struct WebServer {
    visualizer: ArchitectureVisualizer,
    watch_mode: bool,
}

impl WebServer {
    pub fn new(visualizer: ArchitectureVisualizer) -> Self {
        Self {
            visualizer,
            watch_mode: false,
        }
    }
    
    pub fn watch_mode(mut self, enabled: bool) -> Self {
        self.watch_mode = enabled;
        self
    }
    
    /// Start the web server
    pub async fn serve(self, host: &str, port: u16) -> Result<()> {
        let state = WebState::new(self.visualizer);
        state.set_watch_mode(self.watch_mode);
        
        let app = self.create_router(state);
        
        let listener = tokio::net::TcpListener::bind(&format!("{}:{}", host, port)).await?;
        
        info!("🚀 Architecture Visualizer server starting on {}:{}", host, port);
        info!("📊 Open your browser to http://{}:{}", host, port);
        
        if self.watch_mode {
            info!("👀 Watch mode enabled - auto-refreshing on file changes");
        }
        
        axum::serve(listener, app).await?;
        
        Ok(())
    }
    
    /// Create the router with all routes
    fn create_router(&self, state: WebState) -> Router {
        Router::new()
            // Main routes
            .route("/", get(handlers::index_handler))
            .route("/api/architecture", get(handlers::architecture_handler))
            .route("/api/refresh", post(handlers::refresh_handler))
            .route("/api/config", get(handlers::config_handler))
            .route("/api/metrics", get(handlers::metrics_handler))
            
            // WebSocket routes
            .route("/ws", get(websocket::websocket_handler))
            .route("/ws/architecture", get(websocket::architecture_websocket_handler))
            
            // Static files (if needed)
            .route("/static/*path", get(handlers::static_handler))
            
            // Health check
            .route("/health", get(handlers::health_handler))
            
            // Add middleware
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(CompressionLayer::new())
                    .layer(
                        CorsLayer::new()
                            .allow_origin(Any)
                            .allow_methods(Any)
                            .allow_headers(Any)
                    )
            )
            .with_state(state)
    }
}
