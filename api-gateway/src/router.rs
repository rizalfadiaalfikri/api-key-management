use axum::{Json, Router, routing::any, routing::get};
use serde_json::json;

use crate::{proxy, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/{*path}", any(proxy::forward_request))
        .with_state(state)

}

async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "service": "api-gateway",
        "status": "ok",
        "mode": "thin-forward-proxy"
    }))
}