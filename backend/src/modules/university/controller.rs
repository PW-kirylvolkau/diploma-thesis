use axum::{Extension, Json};
use serde_json::{json, Value};
use sqlx::PgPool;

use super::{
    models::{UniError, University},
    storage,
};

#[axum_macros::debug_handler]
pub async fn create_university(
    Extension(pool): Extension<PgPool>,
    Json(uni): Json<University>,
) -> Result<Json<Value>, UniError> {
    storage::create_university(&uni, &pool).await?;
    Ok(Json(json!({ "msg" : "university created successfully" })))
}
