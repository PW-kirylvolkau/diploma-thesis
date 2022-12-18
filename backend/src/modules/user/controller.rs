use axum::{extract, Extension, Json};
use sqlx::PgPool;

use crate::modules::auth::models::Claims;

use super::{
    models::{User, UserError},
    storage,
};

#[axum_macros::debug_handler]
pub async fn user_profile(
    claims: Claims,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<User>, UserError> {
    let user = storage::get_user_by_email(&claims.email, &pool).await?;
    match user {
        Some(user) => Ok(extract::Json(user)),
        None => Err(UserError::UserDoesNotExist),
    }
}
