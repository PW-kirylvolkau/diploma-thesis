use axum::{routing::get, Router};
use sqlx::{PgConnection, Connection, Executor};

#[derive(Debug)]
struct User {
    user_id: i32,
    username: String
}

#[tokio::main]
async fn main() {
    let mut conn = PgConnection::connect("postgresql://postgres:my_password@localhost:54320/diploma").await.unwrap();
    conn.execute(sqlx::query_file!("src/sql/init.sql")).await.unwrap();
    
    let user = sqlx::query_as!(User, "select * from test_table")
        .fetch_one(&mut conn)
        .await
        .unwrap();

    println!("user name: {}", user.username);

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
