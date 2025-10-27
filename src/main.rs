use axum::{
    routing::{get, post},
    response::Json,
    Router
};

use serde::Serialize;
use sqlx::{SqlitePool, FromRow};
use std::net::SocketAddr;


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

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite:trans_banyumas.db")
        .await
        .expect("Failed to create Sqlite database pool");

    let app = Router::new()
        .route("/jadwal", get(view_schedule))
        .with_state(pool);

    println!("Running server on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}