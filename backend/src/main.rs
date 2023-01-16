mod modules;
mod utils;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
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
            std::env::var("RUST_LOG").unwrap_or_else(|_| "backend=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // initialize app
    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/", get(|| async { "hello world" }))
        .route("/register", post(modules::auth::controller::register))
        .route("/login", post(modules::auth::controller::login))
        .route("/profile", get(modules::user::controller::user_profile))
        .route(
            "/uni",
            post(modules::university::controller::create_university),
        )
        .route(
            "/lesson/:id",
            get(modules::lesson::controller::get_lesson_by_id),
        )
        .route("/lesson", post(modules::lesson::controller::create_lesson))
        .route("/course", post(modules::course::controller::create_course))
        .route("/course/:id", get(modules::course::controller::get_course_with_lessons_by_id))
        .layer(cors)
        .layer(Extension(pool));
    let address = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("fail to start server");
}
