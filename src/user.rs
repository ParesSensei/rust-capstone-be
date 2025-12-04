use crate::app_state::AppState;
use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use bcrypt::{DEFAULT_COST, hash, verify};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Serialize)]
pub struct UserResponse {
    pub message: String,
}

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 4, max = 16, message = "username min 4 and max 16 characters"))]
    pub username: String,
    #[validate(length(min = 8, max = 16, message = "password min 8 and max 16 characters"))]
    pub password: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(FromRow)]
pub struct UserSql {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[debug_handler]
pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    if let Err(err) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(UserResponse {
                message: format!("{}", err),
            }),
        );
    }

    let hashed = hash(&payload.password, DEFAULT_COST).unwrap();

    let result = sqlx::query(
        "INSERT INTO users(username, password, email) VALUES ($1, $2, $3)"
    )
        .bind(&payload.username)
        .bind(&hashed)
        .bind(&payload.email)
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(UserResponse {
                message: "Successfully registered user".to_string(),
            }),
        ),

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(UserResponse {
                message: format!("Failed to register user: {}", e),
            }),
        ),
    }
}

#[debug_handler]
pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, UserSql>("SELECT * FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&state.pool)
        .await;

    match result {
        Ok(Some(user)) => {
            if verify(&payload.password, &user.password).unwrap_or(false) {
                (StatusCode::OK, "Logged in").into_response()
            } else {
                (StatusCode::UNAUTHORIZED, "Login failed").into_response()
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}
