use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize)]
pub struct ApiRespinse<T>
where 
    T: ToSchema
{
    pub success: bool,
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}