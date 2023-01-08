use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, sqlx::Type, Debug)]
#[sqlx(type_name = "lesson_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LessonType {
    Attachment,
    Video,
    Text,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Lesson {
    pub id: i32,
    pub name: String,
    pub text: String,
    pub resource_url: Option<String>,
    pub lesson_type: LessonType,
}

pub enum Errors {
    InternalServerError(String),
    LessonDoesntExist,
}

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        let (status, error_msg) = match self {
            Self::InternalServerError(err) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, err),
            Self::LessonDoesntExist => (
                axum::http::StatusCode::NOT_FOUND,
                "lesson couldn't be found".to_owned(),
            ),
        };
        (status, Json(json!({ "error": error_msg }))).into_response()
    }
}
