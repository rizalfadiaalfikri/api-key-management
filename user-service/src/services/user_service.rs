use anyhow::Result;
use bcrypt::{DEFAULT_COST, hash};
use sqlx::PgPool;

use crate::{dto::user_dto::CreateUserDto, errors::app_error::AppError, models::user::User, repositories::user_repository};

pub async fn create_user(pool: &PgPool, payload: CreateUserDto) -> Result<User, AppError> {
    let hashed_password = hash(&payload.password, DEFAULT_COST)
        .map_err(|err| AppError::InternalServerError(err.to_string()))?;

    let payload = CreateUserDto {
        password: hashed_password,
        ..payload
    };

    let user = user_repository::create_user(pool, payload).await?;

    Ok(user)
} 