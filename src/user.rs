use axum::{
    extract::State,
    response::{IntoResponse, Json},
    http::StatusCode,
    debug_handler,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Serialize)]
pub struct UserResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(FromRow)]
pub struct UserSql {
    pub id: i64,
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

#[debug_handler]
pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
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

#[debug_handler]
pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
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
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    }
}
