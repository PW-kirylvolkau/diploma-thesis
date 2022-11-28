use sqlx::PgPool;

use super::models::{UniError, University};

pub async fn create_university(uni: &University, pool: &PgPool) -> Result<(), UniError> {
    let result = sqlx::query("insert into universities (name) values ($1)")
        .bind(&uni.name)
        .execute(pool)
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() < 1 {
                Err(UniError::UniversityCreationError)
            } else {
                Ok(())
            }
        }
        Err(_err) => Err(UniError::UniversityCreationError),
    }
}