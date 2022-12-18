use once_cell::sync::Lazy;

pub mod controller;
pub mod models;

mod storage;

static KEYS: Lazy<models::Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "Your secret here".to_owned());
    models::Keys::new(secret.as_bytes())
});
