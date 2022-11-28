use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub enum Role {
    Admin,
    Teacher,
    Student,
}

#[derive(Deserialize, sqlx::FromRow)]
pub struct User {
    pub email: String,
    pub password: String,
    // pub role: Role,
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub email: String,
    pub exp: u64,
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub enum AuthError {
    // InvalidToken,
    WrongCredential,
    MissingCredential,
    TokenCreation,
    InternalServerError(String),
    UserDoesNotExist,
    UserAlreadyExists,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        // TODO: learn about Cow
        let (status, error_msg) = match self {
            Self::InternalServerError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("error occured: {err}"),
            ),
            // Self::InvalidToken => (StatusCode::BAD_REQUEST, "invalid_token".to_owned()),
            Self::MissingCredential => (StatusCode::BAD_REQUEST, "missing credential".to_owned()),
            Self::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "couldn't create a token".to_owned(),
            ),
            Self::WrongCredential => (StatusCode::UNAUTHORIZED, "wrong credential".to_owned()),
            Self::UserDoesNotExist => (StatusCode::UNAUTHORIZED, "user does not exist".to_owned()),
            Self::UserAlreadyExists => (StatusCode::BAD_REQUEST, "user already exists".to_owned()),
        };
        (status, Json(json!({ "error": error_msg }))).into_response()
    }
}
