mod models;
mod error;
mod controllers;

use dotenv::dotenv;
use axum::{Router, routing::{get, post}, Extension};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // ceremony
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable.");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("couldn't connect to the database");

    // initialize tracing 
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "backend=debug".into())
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // initialize app
    let cors = CorsLayer::new().allow_origin(Any);
    
    let app = Router::new()
        .route("/", get(|| async {"hello world"} ))
        .route("/register", post(controllers::auth::register))
        .layer(cors)
        .layer(Extension(pool));
    let address = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("fail to start server");
}
