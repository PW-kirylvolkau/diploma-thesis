use sqlx::PgPool;

use super::models::{User, UserError};

pub struct UserDbData {
    pub email: String,
    pub role: Option<String>
}

pub async fn get_user_by_email(email: &str, pool: &PgPool) -> Result<Option<User>, UserError> {
    sqlx::query_as!(
        UserDbData,
        r#"select email, role::text from users where email = $1"#,
        &email
    )
    .fetch_optional(pool)
    .await
    // temporary solution for the role retrieval (might not be needed in the future)
    .map(|option| {
        option.map(|user| match user.role {
            Some(role) => User {
                email: user.email,
                role
            },
            None => unreachable!()
        })
    })
    .map_err(|err| {
        tracing::debug!("{}", err.to_string());
        UserError::UserRetrievalError
    })
}
