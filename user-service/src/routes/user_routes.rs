use axum::{Router, middleware, routing::{get, post, put}};

use crate::{handlers::user_handler, state::AppState, utils::jwt_extractor::auth_middleware};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(user_handler::create_user))
        .route("/", get(user_handler::get_users))
        .route("/{id}",
            get(user_handler::get_user_by_id)
            .put(user_handler::update_user_by_id)
            .delete(user_handler::delete_user_by_id)
        )
        .route("/change-password/{id}", put(user_handler::change_password))
        .layer(middleware::from_fn(auth_middleware))
}
