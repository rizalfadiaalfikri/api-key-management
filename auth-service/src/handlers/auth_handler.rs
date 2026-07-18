use anyhow::Result;
use axum::{Json, extract::State};
use validator::Validate;

use crate::{dto::{api_response::ApiResponse, auth_dto::{LoginDto, LoginResponse}}, errors::app_error::AppError, state::AppState};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginDto>
) -> Result<Json<ApiResponse<LoginResponse>>, AppError> {
    payload.validate().map_err(|err| AppError::ValidationError(err.to_string()))?;

    let response = crate::services::auth_service::login(&state.db, payload).await?;

    Ok(Json(ApiResponse {
        success: true,
        status: 200,
        message: "Login successful".to_string(),
        data: Some(response)
    }))
}