use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Course {
    id: i32,
    name: String,
    tile_url: String,
    is_public: bool,
}
