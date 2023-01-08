use sqlx::PgPool;

use super::models::{AuthError, Role, UserCreateDto};

pub async fn get_user_by_email(
    email: &str,
    pool: &PgPool,
) -> Result<Option<UserCreateDto>, AuthError> {
    sqlx::query_as!(
        UserCreateDto,
        r#"select email, password, role as "role: _" from users where email = $1"#,
        &email
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| AuthError::InternalServerError("error when retrieving user".to_owned()))
}

pub async fn create_user(user: &UserCreateDto, pool: &PgPool) -> Result<(), AuthError> {
    let result = sqlx::query!(
        "insert into users (email, password, role) values ($1, $2, $3)",
        &user.email,
        &user.password,
        &user.role as &Role
    )
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
