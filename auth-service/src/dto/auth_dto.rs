use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;


#[derive(Debug,Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginDto {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String
}