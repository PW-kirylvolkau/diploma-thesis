use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Course {
    pub id: i32,
    pub name: String,
    pub tile_url: String,
    pub is_public: bool,
}

#[derive(Serialize, Deserialize, sqlx::Type, Debug)]
#[sqlx(type_name = "lesson_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LessonType {
    Attachment,
    Video,
    Text,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct LessonDto {
    pub id: i32,
    pub name: String,
    pub text: String,
    pub resource_url: Option<String>,
    pub lesson_type: LessonType,
}

#[derive(Serialize, Deserialize)]
pub struct CourseDto {
    pub id: i32,
    pub name: String,
    pub tile_url: String,
    pub is_public: bool,
    pub lessons: Vec<LessonDto>
}