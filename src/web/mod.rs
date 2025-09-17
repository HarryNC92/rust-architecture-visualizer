pub mod server;
pub mod handlers;
pub mod websocket;

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::visualizer::ArchitectureVisualizer;

pub use server::WebServer;

/// Web server state
#[derive(Clone)]
pub struct WebState {
    pub visualizer: Arc<RwLock<ArchitectureVisualizer>>,
    pub watch_mode: bool,
}

impl WebState {
    pub fn new(visualizer: ArchitectureVisualizer) -> Self {
        Self {
            visualizer: Arc::new(RwLock::new(visualizer)),
            watch_mode: false,
        }
    }
    
    pub fn set_watch_mode(&mut self, enabled: bool) {
        self.watch_mode = enabled;
    }
}
