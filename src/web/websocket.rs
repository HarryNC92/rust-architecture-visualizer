use axum::{
    extract::{State, WebSocketUpgrade},
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    web::WebState,
    types::ArchitectureMap,
};

/// WebSocket handler for real-time updates
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<WebState>,
) -> Response {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

/// WebSocket handler specifically for architecture updates
pub async fn architecture_websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<WebState>,
) -> Response {
    ws.on_upgrade(|socket| handle_architecture_websocket(socket, state))
}

/// Handle WebSocket connection
async fn handle_websocket(socket: axum::extract::ws::WebSocket, state: WebState) {
    let (mut sender, mut receiver) = socket.split();
    
    // Send initial architecture data
    if let Ok(architecture) = {
        let visualizer = state.visualizer.read().await;
        visualizer.get_architecture().await
    } {
        let message = json!({
            "type": "architecture",
            "data": architecture
        });
        
        if let Ok(json_str) = serde_json::to_string(&message) {
            let _ = sender.send(axum::extract::ws::Message::Text(json_str)).await;
        }
    }
    
    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(axum::extract::ws::Message::Text(text)) => {
                // Handle client messages
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(msg_type) = data.get("type").and_then(|v| v.as_str()) {
                        match msg_type {
                            "ping" => {
                                let pong = json!({
                                    "type": "pong",
                                    "timestamp": chrono::Utc::now()
                                });
                                
                                if let Ok(json_str) = serde_json::to_string(&pong) {
                                    let _ = sender.send(axum::extract::ws::Message::Text(json_str)).await;
                                }
                            }
                            "refresh" => {
                                // Trigger refresh
                                let mut visualizer = state.visualizer.write().await;
                                if let Ok(architecture) = visualizer.refresh().await {
                                    let message = json!({
                                        "type": "architecture",
                                        "data": architecture
                                    });
                                    
                                    if let Ok(json_str) = serde_json::to_string(&message) {
                                        let _ = sender.send(axum::extract::ws::Message::Text(json_str)).await;
                                    }
                                }
                            }
                            _ => {
                                // Unknown message type
                                let error = json!({
                                    "type": "error",
                                    "message": "Unknown message type"
                                });
                                
                                if let Ok(json_str) = serde_json::to_string(&error) {
                                    let _ = sender.send(axum::extract::ws::Message::Text(json_str)).await;
                                }
                            }
                        }
                    }
                }
            }
            Ok(axum::extract::ws::Message::Close(_)) => {
                break;
            }
            Err(e) => {
                tracing::error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

/// Handle architecture-specific WebSocket connection
async fn handle_architecture_websocket(socket: axum::extract::ws::WebSocket, state: WebState) {
    let (mut sender, mut receiver) = socket.split();
    
    // Send initial architecture data
    if let Ok(architecture) = {
        let visualizer = state.visualizer.read().await;
        visualizer.get_architecture().await
    } {
        let message = json!({
            "type": "architecture",
            "data": architecture
        });
        
        if let Ok(json_str) = serde_json::to_string(&message) {
            let _ = sender.send(axum::extract::ws::Message::Text(json_str)).await;
        }
    }
    
    // If in watch mode, set up file watching
    if state.watch_mode {
        // TODO: Implement file watching and send updates
        // For now, just send periodic updates
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
        
        tokio::spawn(async move {
            loop {
                interval.tick().await;
                
                if let Ok(architecture) = {
                    let visualizer = state.visualizer.read().await;
                    visualizer.get_architecture().await
                } {
                    let message = json!({
                        "type": "architecture",
                        "data": architecture
                    });
                    
                    if let Ok(json_str) = serde_json::to_string(&message) {
                        let _ = sender.send(axum::extract::ws::Message::Text(json_str)).await;
                    }
                }
            }
        });
    }
    
    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(axum::extract::ws::Message::Text(text)) => {
                // Handle client messages
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(msg_type) = data.get("type").and_then(|v| v.as_str()) {
                        match msg_type {
                            "ping" => {
                                let pong = json!({
                                    "type": "pong",
                                    "timestamp": chrono::Utc::now()
                                });
                                
                                if let Ok(json_str) = serde_json::to_string(&pong) {
                                    let _ = sender.send(axum::extract::ws::Message::Text(json_str)).await;
                                }
                            }
                            "refresh" => {
                                // Trigger refresh
                                let mut visualizer = state.visualizer.write().await;
                                if let Ok(architecture) = visualizer.refresh().await {
                                    let message = json!({
                                        "type": "architecture",
                                        "data": architecture
                                    });
                                    
                                    if let Ok(json_str) = serde_json::to_string(&message) {
                                        let _ = sender.send(axum::extract::ws::Message::Text(json_str)).await;
                                    }
                                }
                            }
                            _ => {
                                // Unknown message type
                                let error = json!({
                                    "type": "error",
                                    "message": "Unknown message type"
                                });
                                
                                if let Ok(json_str) = serde_json::to_string(&error) {
                                    let _ = sender.send(axum::extract::ws::Message::Text(json_str)).await;
                                }
                            }
                        }
                    }
                }
            }
            Ok(axum::extract::ws::Message::Close(_)) => {
                break;
            }
            Err(e) => {
                tracing::error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
