use serde::{Deserialize};

#[derive(Deserialize, sqlx::FromRow)]
pub struct User {
    pub email: String,
    pub password: String
}