use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
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

}