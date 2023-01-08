use sqlx::PgPool;

use super::models::{Errors, Lesson, LessonType};

pub struct InsertId {
    id: i32
}

pub async fn create_lesson(lesson: &Lesson, pool: &PgPool) -> Result<i32, Errors> {
    let result = sqlx::query_as!(
        InsertId,
        "insert into lessons (name, text, type, resource_url) values ($1, $2, $3, $4) RETURNING id",
        &lesson.name,
        &lesson.text,
        &lesson.lesson_type as &LessonType,
        lesson.resource_url
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(res) => {
            Ok(res.id)
        }
        Err(_err) => Err(Errors::InternalServerError(
            "error when creating lesson".to_owned(),
        )),
    }
}

pub async fn get_lesson_by_id(id: i32, pool: &PgPool) -> Result<Option<Lesson>, Errors> {
    sqlx::query_as!(
        Lesson,
        r#"select id, name, text, resource_url, type as "lesson_type: _" from lessons where id = $1"#,
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(|error| {
        tracing::error!("on retrieval of lesson with id {} : {}", id, error);
        Errors::InternalServerError("error when retrieving lesson".to_owned())
    })
}

pub struct CourseLessonDto {
    pub course_id: i32,
    pub lesson_id: i32,
    pub seq: i32,
}

pub async fn assign_lesson_to_course(lesson_id: i32, course_id: i32, pool: &PgPool) {
    let last_lesson = sqlx::query_as!(
        CourseLessonDto,
        "select * from course_lessons where course_id = $1 and lesson_id = $2 order by seq desc limit 1",
        course_id,
        lesson_id
    )
    .fetch_optional(pool)
    .await
    .unwrap();

    match last_lesson {
        Some(lesson) => {
            sqlx::query!(
                "insert into course_lessons (course_id, lesson_id, seq) values ($1, $2, $3)",
                course_id,
                lesson_id,
                lesson.seq + 1)
            .execute(pool)
            .await
            .unwrap();
        },
        None => {
            sqlx::query!(
                "insert into course_lessons (course_id, lesson_id, seq) values ($1, $2, $3)",
                course_id,
                lesson_id,
                1)
            .execute(pool)
            .await
            .unwrap();
        }
    }
}
