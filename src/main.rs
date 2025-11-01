use axum::{
    Router,
    routing::{get, post},
};
use sqlx::SqlitePool;
use std::env;

mod app_state;
mod schedule;
mod user;
mod admin;

use crate::app_state::AppState;
use schedule::view_schedule;
use user::{login_user, register_user};
use admin::admin_register_handler;
use admin::admin_login_handler;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let pool = SqlitePool::connect(&db_url)
        .await
        .expect("Failed to create Sqlite database pool");

    let state = AppState { pool };

    let app = Router::new()
        .route("/jadwal", get(view_schedule))
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/admin_register", post(admin_register_handler))
        .route("/admin_login", post(admin_login_handler))
        .with_state(state);

    println!("Running server on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
