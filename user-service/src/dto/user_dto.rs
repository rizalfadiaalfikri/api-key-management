use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;


#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateUserDto {

    #[validate(length(min = 1, max = 255))]
    pub full_name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1, max = 255))]
    pub password: String,

    #[validate(length(min = 1, max = 255))]
    pub role: String,

}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserDto {

    #[validate(length(min = 1, max = 255))]
    pub full_name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1, max = 255))]
    pub password: String,

    #[validate(length(min = 1, max = 255))]
    pub role: String,

    #[validate(length(min = 1, max = 255))]
    pub status: String,

}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserDto {
    pub id: Uuid,
    pub full_name: String,
    pub role: String,
    pub email: String,
    pub password_hash: String,
    pub last_login_at: Option<DateTime<Utc>>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct ChangePasswordDto {

    #[validate(length(min = 1, max = 255))]
    pub old_password: String,

    #[validate(length(min = 1, max = 255))]
    pub new_password: String,
}