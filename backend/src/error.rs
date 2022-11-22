use axum::{response::IntoResponse, http::StatusCode, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    // InvalidToken,
    // WrongCredential,
    MissingCredential,
    // TokenCreation,
    InternalServerError,
    // UserDoesNotExist,
    UserAlreadyExists
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_msg) = match self {
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "error occured"),
            // Self::InvalidToken => (StatusCode::BAD_REQUEST, "invalid_token"),
            Self::MissingCredential => (StatusCode::BAD_REQUEST, "missing credential"),
            // Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "couldn't create a token"),
            // Self::WrongCredential => (StatusCode::UNAUTHORIZED, "wrong credential"),
            // Self::UserDoesNotExist => (StatusCode::UNAUTHORIZED, "user does not exist"),
            Self::UserAlreadyExists => (StatusCode::BAD_REQUEST, "user already exists"),
        };
        (status, Json(json!({ "error": error_msg }))).into_response()
    }
}