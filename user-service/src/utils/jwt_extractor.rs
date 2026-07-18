use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    middleware::Next,
    response::{IntoResponse, Response},
    extract::Request,
};

use crate::{errors::app_error::AppError, settings, utils::jwt};

pub struct AuthClaims {
    pub user_id: String,
}

impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .ok_or_else(|| AppError::Unauthorized.into_response())?;

        let auth_str = auth_header
            .to_str()
            .map_err(|_| AppError::Unauthorized.into_response())?;

        // Parse "Bearer <token>"
        if !auth_str.starts_with("Bearer ") {
            return Err(AppError::Unauthorized.into_response());
        }

        let token = &auth_str[7..];

        // Verify token
        let token_data = jwt::decode_raw_token(token, settings::jwt_secret())
            .map_err(|_| AppError::Unauthorized.into_response())?;

        Ok(AuthClaims {
            user_id: token_data.claims.user_id,
        })
    }
}

/// Middleware that rejects requests without a valid Authorization: Bearer <token> header.
pub async fn auth_middleware(request: Request, next: Next) -> Response {
    let (mut parts, body) = request.into_parts();
    match AuthClaims::from_request_parts(&mut parts, &()).await {
        Ok(_) => next.run(Request::from_parts(parts, body)).await,
        Err(rejection) => rejection,
    }
}
