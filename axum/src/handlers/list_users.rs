use axum::response::Json;
use serde_json::Value;

use crate::errors::api_errors::ApiError;

pub async fn list_users() -> Result<Json<Value>, ApiError> {
    Err(ApiError::InternalError)
}
