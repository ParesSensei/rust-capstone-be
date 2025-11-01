use axum::extract::State;
use axum::http::StatusCode;
use axum::{debug_handler, Json};
use axum::response::IntoResponse;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::app_state::AppState;
use serde::{Deserialize, Serialize};
use crate::user::UserSql;

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
pub async fn admin_register_handler(
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

pub async fn admin_login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, UserSql>("SELECT * FROM admin WHERE username = ?")
        .bind(&payload.username)
        .fetch_optional(&state.pool)
        .await;

    match result {
        Ok(Some(admin)) => {
            if verify(&payload.password, &admin.password).unwrap_or(false) {
                (StatusCode::OK, "Logged in").into_response()
            } else {
                (StatusCode::UNAUTHORIZED, "failed to login").into_response()
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(err) => {
                eprintln!("DB error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }