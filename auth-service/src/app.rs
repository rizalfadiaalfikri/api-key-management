use axum::Router;
use tower_http::trace::TraceLayer;

use crate::state::{AppState};

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/health", axum::routing::get(|| async { "OK" }))
        // .nest("/api/auth", router)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}