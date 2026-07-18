use anyhow::Result;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{dto::{api_response::ApiResponse, user_dto::{ChangePasswordDto, CreateUserDto, UpdateUserDto, UserDto}}, errors::app_error::AppError, models::user::User, services::user_service, state::AppState};

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

#[derive(Deserialize)]
pub struct GetUsersParams {
    email: Option<String>,
}

pub async fn get_users(
    State(state): State<AppState>,
    params: Query<GetUsersParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    if let Some(email) = &params.email {
        let user = user_service::get_user_by_email(&state.db, email).await?;
        let api = ApiResponse::<UserDto> {
            success: true,
            status: 200,
            message: "User retrieved successfully".to_string(),
            data: Some(user),
        };
        return Ok(Json(serde_json::to_value(api).map_err(|e| AppError::InternalServerError(e.to_string()))?));
    }

    let users = user_service::get_all_users(&state.db).await?;
    let api = ApiResponse::<Vec<UserDto>> {
        success: true,
        status: 200,
        message: "Users retrieved successfully".to_string(),
        data: Some(users),
    };
    Ok(Json(serde_json::to_value(api).map_err(|e| AppError::InternalServerError(e.to_string()))?))
}


pub async fn update_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserDto>
) -> Result<Json<ApiResponse<User>>, AppError>{
    payload.validate()
        .map_err(|err| AppError::ValidationError(err.to_string()))?;

    let user = user_service::update_user_by_id(&state.db, id, payload).await?;

    Ok(Json(ApiResponse::<User> {
        success: true,
        status: 200,
        message: "User updated successfully".to_string(),
        data: Some(user),
    }))
}

pub async fn delete_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<Json<ApiResponse<User>>, AppError> {
    let user = user_service::delete_user_by_id(&state.db, id).await?;

    Ok(Json(ApiResponse::<User> {
        success: true,
        status: 200,
        message: "User deleted successfully".to_string(),
        data: Some(user),
    }))
}

pub async fn change_password(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ChangePasswordDto>
) -> Result<Json<ApiResponse<User>>, AppError> {
    payload.validate()
        .map_err(|err| AppError::ValidationError(err.to_string()))?;

    let user = user_service::change_password(&state.db, id, payload).await?;

    Ok(Json(ApiResponse::<User> {
        success: true,
        status: 200,
        message: "Password changed successfully".to_string(),
        data: Some(user),
    }))
}