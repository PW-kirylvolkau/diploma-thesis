use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;

#[derive(Deserialize, sqlx::FromRow)]
pub struct University {
    pub name: String,
}

pub enum UniError {
    UniversityCreationError,
}

impl IntoResponse for UniError {
    fn into_response(self) -> Response {
        let (status, error_msg) = match self {
            Self::UniversityCreationError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to create the univerity.",
            ),
        };
        (status, error_msg).into_response()
    }
}
