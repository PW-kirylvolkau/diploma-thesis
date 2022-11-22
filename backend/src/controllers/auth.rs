use axum::{Json, Extension};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{
    error::AppError,
    models
};

pub async fn register(
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