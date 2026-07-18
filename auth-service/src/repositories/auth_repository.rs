use sqlx::PgPool;

use crate::{dto::user_dto::UserDto, errors::app_error::AppError};

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