use axum::{Extension, Json};
use jsonwebtoken::{encode, Header};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{utils::get_timestamp_8_hours_from_now};

use super::{
    models::AuthError,
    models::{Claims, User},
    storage, KEYS,
};

pub async fn login(
    Json(credentials): Json<User>,
    Extension(pool): Extension<PgPool>,
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

pub async fn register(
    // TODO: read about pattern matching in parameters
    Json(credentials): Json<User>,
    Extension(pool): Extension<PgPool>,
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
