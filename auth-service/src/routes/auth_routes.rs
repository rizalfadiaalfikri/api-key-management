use axum::{Router, routing::{get, post}};

use crate::state::AppState;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", get(|| async { "login" }))
}