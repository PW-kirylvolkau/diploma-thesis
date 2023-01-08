use sqlx::PgPool;

use super::models::Course;

pub struct InsertId {
    id: i32
}

pub async fn create_course(course: Course, pool: &PgPool) -> Result<i32, &str> {
    let result = sqlx::query_as!(
        InsertId,
        "insert into courses (name, tile_url, is_public) values ($1, $2, $3) RETURNING id",
        &course.name,
        &course.tile_url,
        &course.is_public,
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(res) => {
            Ok(res.id)
        }
        Err(_err) => Err("error when creating course"),
    }
}

pub async fn assign_course_to_university(uni_id: i32, course_id: i32, pool: &PgPool) {
    sqlx::query!(
        "insert into university_courses (uni_id, course_id) values ($1, $2)",
        uni_id,
        course_id)
    .execute(pool)
    .await
    .unwrap();
}