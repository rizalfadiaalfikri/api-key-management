use anyhow::Result;
use axum::{Json, extract::State, http::StatusCode};
use validator::Validate;

use crate::{dto::{api_response::ApiResponse, user_dto::CreateUserDto}, errors::app_error::AppError, models::user::User, services::user_service, state::AppState};

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>
) -> Result<(StatusCode, Json<ApiResponse<User>>), AppError> {
    payload.validate()
        .map_err(|err| AppError::ValidationError(err.to_string()))?;

    let user = user_service::create_user(&state.db, payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::<User> {
            success: true,
            status: 201,
            message: "User created successfully".to_string(),
            data: Some(user),
        }),
    ))
}