use axum::{
    routing::{get, post},
    response::Json,
    Router, debug_handler
};

use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow, Pool, Executor, query};
use std::net::SocketAddr;
use axum::extract::State;
use axum::http::StatusCode;
use sqlx::sqlite::SqliteQueryResult;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Serialize, FromRow)]
struct Schedule {
    id: i64,
    koridor_id: i64,
    halte_id: i64,
    departure: String,
    day: String,
}

async fn view_schedule(pool: axum::extract::State<SqlitePool>) -> Json<Vec<Schedule>> {
    let schedules = sqlx::query_as::<_, Schedule>("SELECT * FROM jadwal")
        .fetch_all(&pool.0)
        .await
        .expect("Gagal ambil jadwal");

    Json(schedules)
}


#[derive(Serialize)]
struct UserResponse{
    message: String,
}

#[derive(Deserialize)]
struct RegisterRequest{
    username: String,
    password: String,
}

#[debug_handler]
async fn register_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<RegisterRequest>
) -> Json<UserResponse> {

    let hashed = hash(&payload.password, DEFAULT_COST).unwrap();

    let result = sqlx::query!(
        "INSERT INTO user(username, password) VALUES (?1, ?2)",
        payload.username,
        hashed
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Json(UserResponse {
            message: "Successfully registered user".to_string(),
        }),
        Err(e) => Json(UserResponse {
            message: format!("Failed to register user: {}", e),
        }),
    }
}


#[tokio::main]
async fn main() {

    dotenvy::dotenv().ok();

    let pool = SqlitePool::connect("sqlite:trans_banyumas.db")
        .await
        .expect("Failed to create Sqlite database pool");

    let app = Router::new()
        .route("/jadwal", get(view_schedule))
        .route("/register", post(register_user))
        .with_state(pool);

    println!("Running server on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}