use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub role: String,
}

pub enum UserError {
    UserRetrievalError,
    UserDoesNotExist,
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            Self::UserRetrievalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "failed to retrieve user")
            }
            Self::UserDoesNotExist => (StatusCode::BAD_REQUEST, "user does not exist"),
        };
        (status, Json(json!({ "msg": err_msg }))).into_response()
    }
}
