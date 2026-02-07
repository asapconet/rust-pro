use axum::{extract::Path, response::Json};
use serde_json::{json, Value};

use crate::errors::ApiError;

pub async fn get_user(Path(id): Path<u32>) -> Result<Json<Value>, ApiError> {
    if id > 100 {
        return Err(ApiError::InvalidInput("ID too large".to_string()));
    }
    Ok(Json(json!({"id":id, "name":"User"})))
}
