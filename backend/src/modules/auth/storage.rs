use sqlx::PgPool;

use super::models::{AuthError, User};

pub async fn get_user_by_email(email: &str, pool: &PgPool) -> Result<Option<User>, AuthError> {
    sqlx::query_as::<_, User>("select email, password from users where email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
        .map_err(|_| AuthError::InternalServerError("error when retrieving user".to_owned()))
}

pub async fn create_user(user: &User, pool: &PgPool) -> Result<(), AuthError> {
    let result = sqlx::query("insert into users (email, password) values ($1, $2)")
        .bind(&user.email)
        .bind(&user.password)
        .execute(pool)
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() < 1 {
                Err(AuthError::InternalServerError(
                    "couldn't create user".to_owned(),
                ))
            } else {
                Ok(())
            }
        }
        Err(_err) => Err(AuthError::InternalServerError(
            "error when creating user".to_owned(),
        )),
    }
}