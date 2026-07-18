use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct Credentials {
    pub user_id: String,
    pub password_hash: String,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>
}