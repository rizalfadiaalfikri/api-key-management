use anyhow::Result;
use sqlx::{PgPool};
use uuid::Uuid;

use crate::dto::user_dto::{CreateUserDto, UpdateUserDto, UserDto};
use crate::errors::app_error::AppError;
use crate::models::user::User;

pub async fn create_user(pool: &PgPool, payload: CreateUserDto) -> Result<User, AppError> {
    let mut tx = pool.begin().await?;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (full_name, email, role, status, created_at, updated_at)
        VALUES ($1, $2, $3, 'active', NOW(), NOW())
        RETURNING *
        "#,
        payload.full_name,
        payload.email,
        payload.role,
    )
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO credentials (user_id, password_hash, created_at, updated_at)
        VALUES ($1, $2, NOW(), NOW())
        "#,
        user.id,
        payload.password,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(user)
}

pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<UserDto>, AppError> {
    let user = sqlx::query_as::<_, UserDto>(
        r#"
        SELECT u.id, u.full_name, u.role, u.email, c.password_hash, c.last_login_at, u.status, u.created_at, u.updated_at
        FROM users u
        LEFT JOIN credentials c ON c.user_id = u.id
        WHERE u.id = $1
        "#
    ).bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<UserDto>, AppError> {
    let user = sqlx::query_as::<_, UserDto>(
        r#"
        SELECT u.id, u.full_name, u.role, u.email, c.password_hash, c.last_login_at, u.status, u.created_at, u.updated_at
        FROM users u
        LEFT JOIN credentials c ON c.user_id = u.id
        WHERE u.email = $1
        "#
    ).bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<UserDto>, AppError> {
    let users = sqlx::query_as::<_, UserDto>(
        r#"
        SELECT u.id, u.full_name, u.role, u.email, c.password_hash, c.last_login_at, u.status, u.created_at, u.updated_at
        FROM users u
        LEFT JOIN credentials c ON c.user_id = u.id
        "#
    ).fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn update_user_by_id(pool: &PgPool, id: Uuid, payload: UpdateUserDto) -> Result<User, AppError> {
    let mut tx = pool.begin().await?;

    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET full_name = $1, email = $2, role = $3, status = $4
        WHERE id = $5
        RETURNING *
        "#,
        payload.full_name,
        payload.email,
        payload.role,
        payload.status,
        id,
    )
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query!(
        r#"
        UPDATE credentials
        SET password_hash = $1
        WHERE user_id = $2
        "#,
        payload.password,
        id,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(user)
}