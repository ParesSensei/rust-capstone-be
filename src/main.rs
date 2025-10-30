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
use axum::response::IntoResponse;
use sqlx::sqlite::SqliteQueryResult;
use bcrypt::{hash, DEFAULT_COST, verify};


#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

#[derive(Serialize, FromRow)]
struct Schedule {
    id: i64,
    koridor_id: i64,
    halte_id: i64,
    departure: String,
    day: String,
}

async fn view_schedule(State(state): State<AppState>) -> Json<Vec<Schedule>> {
    let schedules = sqlx::query_as::<_, Schedule>("SELECT * FROM jadwal")
        .fetch_all(&state.pool)
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
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>
) -> Json<UserResponse> {

    let hashed = hash(&payload.password, DEFAULT_COST).unwrap();

    let result = sqlx::query!(
        "INSERT INTO user(username, password) VALUES (?1, ?2)",
        payload.username,
        hashed
    )
    .execute(&state.pool)
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

#[derive(Deserialize)]
struct LoginRequest{
    username: String,
    password: String,
}

#[derive(FromRow)]
struct UserSql{
    id: i64,
    username: String,
    password: String,
}

#[debug_handler]
async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>
) -> impl IntoResponse {
    // let hashed = hash(&payload.password, DEFAULT_COST).unwrap();

    let result = sqlx::query_as::<_, UserSql>("SELECT * FROM user WHERE username = ?")
        .bind(&payload.username)
        .fetch_optional(&state.pool)
        .await;
    match result {
        Ok(Some(user)) => {
            if verify(payload.password, &user.password).unwrap_or(false) {
                (StatusCode::OK, "Logged in").into_response()
            } else {
                (StatusCode::UNAUTHORIZED, "Login failed").into_response()
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, "user not found").into_response(),
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}


#[tokio::main]
async fn main() {

    dotenvy::dotenv().ok();

    let pool = SqlitePool::connect("sqlite:trans_banyumas.db")
        .await
        .expect("Failed to create Sqlite database pool");

    let state = AppState{pool};

    let app = Router::new()
        .route("/jadwal", get(view_schedule))
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .with_state(state);

    println!("Running server on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}