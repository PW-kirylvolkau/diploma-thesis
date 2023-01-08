use core::panic;

use axum::{
    extract::{multipart::Field, Multipart, Path},
    Extension, Json,
};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::modules::lesson::storage;

use super::{
    blob,
    models::{Errors, Lesson, LessonType},
};

#[axum_macros::debug_handler]
pub async fn create_lesson(
    Extension(pool): Extension<PgPool>,
    mut multipart: Multipart,
) -> Result<Json<Value>, Errors> {
    // pub name: String,
    // pub text: String,
    // pub resource_url: Option<String>,
    let mut lesson_name: Option<String> = None;
    let mut lesson_type: Option<LessonType> = None;
    let mut text: Option<String> = None;
    let mut resource_url: Option<String> = None;
    let mut course_id: Option<i32> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        // TODO: replace unwraps with actual error handling
        let name = field.name().unwrap().to_string().to_owned();

        match name.as_str() {
            "File" => {
                let file = handle_file_field(field).await;
                lesson_type = Some(file.0);
                resource_url = file.1;
            }
            "Text" => text = Some(field.text().await.unwrap()),
            "Name" => lesson_name = Some(field.text().await.unwrap()),
            "CourseId" => course_id = Some(field.text().await.unwrap().parse::<i32>().unwrap()),
            _ => panic!("Shiiiit"),
        }
    }
    if lesson_type.is_none() {
        lesson_type = Some(LessonType::Text)
    };
    let lesson = Lesson {
        id: 0,
        name: lesson_name.unwrap(),
        text: text.unwrap(),
        resource_url,
        lesson_type: lesson_type.unwrap(),
    };
    let lesson_id = storage::create_lesson(&lesson, &pool).await?;
    storage::assign_lesson_to_course(lesson_id, course_id.unwrap(), &pool).await;
    Ok(Json(json!({ "msg" : "lesson created successfully" })))
}

#[axum_macros::debug_handler]
pub async fn get_lesson_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Lesson>, Errors> {
    let lesson = storage::get_lesson_by_id(id, &pool).await?;
    match lesson {
        Some(lesson) => Ok(axum::extract::Json(lesson)),
        None => Err(Errors::LessonDoesntExist),
    }
}

async fn handle_file_field<'a>(field: Field<'_>) -> (LessonType, Option<String>) {
    match field.content_type().unwrap() {
        attachment if attachment.starts_with("application/") => (
            LessonType::Attachment,
            blob::upload_file_to_blob(field).await,
        ),
        video if video.starts_with("video/") => {
            (LessonType::Video, blob::upload_file_to_blob(field).await)
        }
        _ => (LessonType::Text, None),
    }
}
