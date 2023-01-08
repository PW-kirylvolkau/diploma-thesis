use axum::{Extension, Json, extract::{Multipart, Path}};
use serde_json::{Value, json};
use sqlx::PgPool;

use crate::modules::{course::{models::Course, storage}, blob::upload_file_to_blob};

use super::models::CourseDto;

#[axum_macros::debug_handler]
pub async fn create_course(
    Extension(pool): Extension<PgPool>,
    mut multipart: Multipart,
) -> Result<Json<Value>, String> {
    // pub name: String,
    // pub text: String,
    // pub resource_url: Option<String>,
    let mut course_name: Option<String> = None;
    let mut course_tile: Option<String> = None;
    let mut uni_id: Option<i32> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        // TODO: replace unwraps with actual error handling
        let name = field.name().unwrap().to_string().to_owned();

        match name.as_str() {
            "File" => {
                let file = upload_file_to_blob(field).await.unwrap();
                course_tile = Some(file);
            }
            "Name" => course_name = Some(field.text().await.unwrap()),
            "UniId" => uni_id = Some(field.text().await.unwrap().parse::<i32>().unwrap()),
            _ => panic!("Shiiiit"),
        }
    }

    let course = Course {
        id: 0,
        name: course_name.unwrap(),
        tile_url: course_tile.unwrap(),
        is_public: false
    };
    let course_id = storage::create_course(course, &pool).await?;
    storage::assign_course_to_university(course_id, uni_id.unwrap(), &pool).await;
    Ok(Json(json!({ "msg" : "lesson created successfully" })))
}

#[axum_macros::debug_handler]
pub async fn get_course_with_lessons_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<CourseDto>, String> {
    let course = storage::get_course_with_lessons_by_id(id, &pool).await;
    Ok(axum::extract::Json(course))
}