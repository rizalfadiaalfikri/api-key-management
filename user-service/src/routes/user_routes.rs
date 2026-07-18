use axum::{Router, routing::{get, post}};

use crate::{handlers::user_handler, state::AppState};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(user_handler::create_user))
        .route("/", get(user_handler::get_users))
        .route("/{id}", 
            get(user_handler::get_user_by_id)
            .put(user_handler::update_user_by_id)
            .delete(user_handler::delete_user_by_id)
        )
}
