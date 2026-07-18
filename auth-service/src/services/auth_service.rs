use bcrypt::verify;
use sqlx::PgPool;

use crate::{dto::{auth_dto::{LoginDto, LoginResponse}, user_dto::UserDto}, errors::app_error::AppError, settings, utils::jwt};

pub async fn login(pool: &PgPool, payload: LoginDto) -> Result<LoginResponse, AppError> {
    // get user by email
    let user: Option<UserDto> = crate::repositories::auth_repository::get_user_by_email(pool, &payload.email).await?;

    let user = user.ok_or(AppError::NotFound)?;

    // check password
    let password_valid = verify(&payload.password, &user.password_hash).map_err(|err| AppError::InternalServerError(err.to_string()))?;

    if !password_valid {
        return Err(AppError::ValidationError("Invalid email or password".to_string()));
    }

    let expiration = settings::jwt_expiration() as u64;

    // Generate a JWT token
    let token = jwt::generate_token(
        user.id.to_string(),
        settings::jwt_secret(),
        expiration as i64,
    )
    .map_err(|err| AppError::InternalServerError(err.to_string()))?;

    let response = LoginResponse {
        access_token: token,
        expires_in: expiration,
        token_type: "Bearer".to_string(),
    };

    Ok(response)
}