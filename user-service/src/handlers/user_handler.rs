use anyhow::Result;
use axum::{Json, extract::{Path, State}, http::StatusCode};
use uuid::Uuid;
use validator::Validate;

use crate::{dto::{api_response::ApiResponse, user_dto::{CreateUserDto, UserDto}}, errors::app_error::AppError, models::user::User, services::user_service, state::AppState};

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

pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<Json<ApiResponse<UserDto>>, AppError> {
    let user = user_service::get_user_by_id(&state.db, id).await?;

    Ok(Json(ApiResponse::<UserDto> {
        success: true,
        status: 200,
        message: "User retrieved successfully".to_string(),
        data: Some(user),
    }))
}