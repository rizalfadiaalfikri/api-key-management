use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, encode, decode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("JWT error: {0}")]
    JsonWebToken(#[from] jsonwebtoken::errors::Error),
    #[error("Token expired")]
    TokenExpired,
    #[error("Invalid token")]
    InvalidToken,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

impl Claims {
    pub fn new(user_id: String, exp_secs: i64) -> Self {
        let now = Utc::now();
        Claims {
            user_id,
            exp: (now + Duration::seconds(exp_secs as i64)).timestamp(),
            iat: now.timestamp(),
        }
    }
}

pub fn generate_token(user_id: String, secret: &str, exp_secs: i64) 
    -> Result<String, JwtError> {
        let claims = Claims::new(user_id, exp_secs);
        encode(
            &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(Into::into)
}

pub fn verify_token(
    token: &str,
    secret: &str,
) -> Result<TokenData<Claims>, JwtError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| {
        if matches!(*e.kind(), jsonwebtoken::errors::ErrorKind::ExpiredSignature) {
            JwtError::TokenExpired
        } else {
            JwtError::InvalidToken
        }
    })
}

pub fn decode_raw_token(
    token: &str,
    secret: &str
) -> Result<TokenData<Claims>, JwtError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(Into::into)
}