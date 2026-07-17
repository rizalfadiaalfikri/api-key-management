use axum::Router;

use crate::state::AppState;

pub fn user_routes() -> Router<AppState> {
    Router::new().route("/", axum::routing::get(|| async { "Users" }))
}