use axum::{Json, Extension};
use jsonwebtoken::{encode, Header};
use serde_json::{json, Value};
use sqlx::{PgPool};

use crate::{
    error::AppError,
    models::{self, auth::Claims}, utils::get_timestamp_8_hours_from_now, KEYS
};

pub async fn login(
    Json(credentials): Json<models::auth::User>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, AppError> {
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredential);
    }

    let user = sqlx::query_as::<_, models::auth::User>(
        "select email, password from users where email = $1"
    )
    .bind(&credentials.email)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        AppError::InternalServerError
    })?;

    if let Some(user) = user {
        if user.password != credentials.password {
            Err(AppError::WrongCredential)
        } else {
            let claims = Claims {
                email: credentials.email.to_owned(),
                exp: get_timestamp_8_hours_from_now(),
            };
            let token = encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| AppError::TokenCreation)?;
            Ok(Json(json!({ "access_token" : token, "type": "Bearer"})))
        }
    } else {
        Err(AppError::UserDoesNotExist)
    }
}

pub async fn register(
    // TODO: read about pattern matching in parameters
    Json(credentials): Json<models::auth::User>,
    Extension(pool): Extension<PgPool>
) -> Result<Json<Value>, AppError>
{
    // check if email or password is a blank string 
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredential);
    }

    // get the user for the email from the database 
    let user = sqlx::query_as::<_, models::auth::User>(
        "select email, password from users where email = $1")
        .bind(&credentials.email)
        .fetch_optional(&pool)
        .await
        .map_err(|err| {
            dbg!(err);
            AppError::InternalServerError
        })?;
    
    if user.is_some() {
        // if a user with email already exists send error 
        return Err(AppError::UserAlreadyExists);
    }

    let result = sqlx::query(
        "insert into users (email, password) values ($1, $2)")
        .bind(&credentials.email)
        .bind(&credentials.password)
        .execute(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    if result.rows_affected() < 1 {
        Err(AppError::InternalServerError)
    } else {
        Ok(Json(json!({"msg" : "registered successfully" })))
    }
}