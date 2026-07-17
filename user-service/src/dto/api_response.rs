use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T>
where
    T: ToSchema,
{
    pub success: bool,
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}