use axum::{extract::Json, Extension};
use jsonwebtoken::{encode, Header};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{utils::get_timestamp_8_hours_from_now, modules::auth::models::Role};

use super::{
    models::AuthError,
    models::{Claims, UserCreateDto},
    storage, KEYS,
};

#[axum_macros::debug_handler]
pub async fn login(
    Extension(pool): Extension<PgPool>,
    Json(credentials): Json<UserCreateDto>,
) -> Result<Json<Value>, AuthError> {
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AuthError::MissingCredential);
    }

    let user = storage::get_user_by_email(&credentials.email, &pool).await?;

    match user {
        Some(user) => {
            if user.password != credentials.password {
                Err(AuthError::WrongCredential)
            } else {
                let claims = Claims {
                    email: credentials.email.to_owned(),
                    role: Role::Student, // TODO: move out into separate student module
                    exp: get_timestamp_8_hours_from_now(),
                };
                let token = encode(&Header::default(), &claims, &KEYS.encoding)
                    .map_err(|_| AuthError::TokenCreation)?;
                Ok(Json(json!({ "access_token" : token, "type": "Bearer"})))
            }
        }
        None => Err(AuthError::UserDoesNotExist),
    }
}

#[axum_macros::debug_handler]
pub async fn register(
    // TODO: read about pattern matching in parameters
    Extension(pool): Extension<PgPool>,
    Json(credentials): Json<UserCreateDto>,
) -> Result<Json<Value>, AuthError> {
    // check if email or password is a blank string
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AuthError::MissingCredential);
    }

    // get the user for the email from the database
    let user = storage::get_user_by_email(&credentials.email, &pool).await?;
    match user {
        Some(_) => Err(AuthError::UserAlreadyExists),
        None => {
            storage::create_user(&credentials, &pool).await?;
            Ok(Json(json!({"msg" : "registered successfully" })))
        }
    }
}
