use axum::{Router, routing::{post}};

use crate::{handlers::auth_handler, state::AppState};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth_handler::login))
}