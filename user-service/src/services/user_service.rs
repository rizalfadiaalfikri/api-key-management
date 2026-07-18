use anyhow::Result;
use bcrypt::{DEFAULT_COST, hash};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{dto::user_dto::{CreateUserDto, UserDto}, errors::app_error::AppError, models::user::User, repositories::user_repository};

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

pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<UserDto, AppError> {
    let user = user_repository::get_user_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(user)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<UserDto, AppError> {
    let user = user_repository::get_user_by_email(pool, email)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(user)
}

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<UserDto>, AppError> {
    let users = user_repository::get_all_users(pool)
        .await?;

    Ok(users)
}