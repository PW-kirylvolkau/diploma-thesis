use axum::{Extension, Json};
use serde_json::{Value, json};
use sqlx::PgPool;

use super::{models::{University, UniError}, storage};

pub async fn create_university(
    Json(uni): Json<University>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, UniError> {
    storage::create_university(&uni, &pool).await?;
    Ok(Json(json!({ "msg" : "university created successfully" })))
}
