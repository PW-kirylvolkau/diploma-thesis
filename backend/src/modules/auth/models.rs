use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt, TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::KEYS;

#[derive(Deserialize, Serialize, sqlx::Type, Debug)]
#[sqlx(type_name = "role", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    Admin,
    Teacher,
    Student,
}

#[derive(Deserialize, sqlx::FromRow)]
pub struct UserCreateDto {
    pub email: String,
    pub password: String,
    pub role: Role,
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub email: String,
    pub role: Role,
    pub exp: u64,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub enum AuthError {
    InvalidToken,
    WrongCredential,
    MissingCredential,
    TokenCreation,
    InternalServerError(String),
    UserDoesNotExist,
    UserAlreadyExists,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        // TODO: learn about Cow
        let (status, error_msg) = match self {
            Self::InternalServerError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("error occured: {err}"),
            ),
            Self::InvalidToken => (StatusCode::BAD_REQUEST, "invalid_token".to_owned()),
            Self::MissingCredential => (StatusCode::BAD_REQUEST, "missing credential".to_owned()),
            Self::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "couldn't create a token".to_owned(),
            ),
            Self::WrongCredential => (StatusCode::UNAUTHORIZED, "wrong credential".to_owned()),
            Self::UserDoesNotExist => (StatusCode::UNAUTHORIZED, "user does not exist".to_owned()),
            Self::UserAlreadyExists => (StatusCode::BAD_REQUEST, "user already exists".to_owned()),
        };
        (status, Json(json!({ "error": error_msg }))).into_response()
    }
}
