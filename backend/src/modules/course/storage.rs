use sqlx::PgPool;

use super::models::Course;
use super::models::CourseDto;
use super::models::LessonDto;

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

pub async fn get_course_with_lessons_by_id(id: i32, pool: &PgPool) -> CourseDto {
    let lessons = sqlx::query_as!(
        LessonDto,
        r#"select distinct l.id, l.name, l.text, l.resource_url, l.type as "lesson_type: _" from lessons l
           join course_lessons cl on cl.lesson_id = l.id
           where cl.course_id = $1"#,
        id)
        .fetch_all(pool)
        .await
        .unwrap();
    let course = sqlx::query_as!(Course, "select * from courses where id = $1", id)
        .fetch_one(pool)
        .await
        .unwrap();
    
    CourseDto {
        id: course.id,
        name: course.name,
        tile_url: course.tile_url,
        is_public: course.is_public,
        lessons
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