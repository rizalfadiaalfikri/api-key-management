use axum::Router;
use tower_http::trace::TraceLayer;

use crate::{routes::{auth_routes}, state::AppState};

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/health", axum::routing::get(|| async { "OK" }))
        .nest("/api/auth", auth_routes::auth_routes())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}