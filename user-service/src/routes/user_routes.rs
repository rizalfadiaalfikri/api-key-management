use axum::{Router, routing::post};

use crate::{handlers::user_handler, state::AppState};

pub fn user_routes() -> Router<AppState> {
    Router::new().route("/", post(user_handler::create_user))
}