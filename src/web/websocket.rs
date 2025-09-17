use axum::{
    extract::State,
    response::Response,
    http::StatusCode,
};

use crate::web::WebState;

/// WebSocket handler for real-time updates (placeholder)
pub async fn websocket_handler(
    State(_state): State<WebState>,
) -> Result<Response, StatusCode> {
    // For now, return a simple message indicating WebSocket is not implemented
    Ok(Response::builder()
        .status(StatusCode::NOT_IMPLEMENTED)
        .body("WebSocket support not yet implemented".into())
        .unwrap())
}

/// WebSocket handler specifically for architecture updates (placeholder)
pub async fn architecture_websocket_handler(
    State(_state): State<WebState>,
) -> Result<Response, StatusCode> {
    // For now, return a simple message indicating WebSocket is not implemented
    Ok(Response::builder()
        .status(StatusCode::NOT_IMPLEMENTED)
        .body("WebSocket support not yet implemented".into())
        .unwrap())
}
