use crate::{models::auth::Claims, error::AppError};

pub async fn user_profile(claims: Claims) -> Result<axum::Json<serde_json::Value>, AppError> {
    Ok(axum::Json(serde_json::json!({"email" : claims.email})))
}