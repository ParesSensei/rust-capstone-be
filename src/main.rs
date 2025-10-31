use axum::{
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;

mod user;
mod schedule;

use user::{AppState, register_user, login_user};
use schedule::view_schedule;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = SqlitePool::connect("sqlite:trans_banyumas.db")
        .await
        .expect("Failed to create Sqlite database pool");

    let state = AppState { pool };

    let app = Router::new()
        .route("/jadwal", get(view_schedule))
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .with_state(state);

    println!("Running server on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
