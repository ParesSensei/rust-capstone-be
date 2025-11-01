use axum::extract::State;
use axum::http::StatusCode;
use axum::{debug_handler, Json};
use bcrypt::{hash, DEFAULT_COST};
use crate::app_state::AppState;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RegisterResponse {
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[debug_handler]
pub async fn admin_login_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Json<RegisterResponse> {

    let hashed = hash(&payload.password, DEFAULT_COST).unwrap();

    let result = sqlx::query!(
        "INSERT INTO admin(username, password) VALUES (?1, ?2)",
        payload.username,
        hashed
    )
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => {Json(RegisterResponse {
            message: "Success create new admin".to_string(),
            })
        },
        Err(err) => { Json(RegisterResponse {
            message: "Failed create new admin".to_string(),
            })
        }
    }
}