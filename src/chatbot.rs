use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::app_state::AppState;

// 1. Struct untuk menerima data dari Frontend (Laporan Chat)
#[derive(Deserialize)]
pub struct ChatLogRequest {
    pub question: String,
    pub answer: String,
}

// 2. Struct untuk mengirim Data Statistik ke Admin Dashboard
#[derive(Serialize, FromRow)]
pub struct ChatStats {
    pub total_chats: i64,
    pub total_questions: i64, // Sama dengan total chat
}

// --- API 1: SIMPAN LOG CHAT (Dipanggil saat user kirim pesan) ---
pub async fn save_chat_log(
    State(state): State<AppState>,
    Json(payload): Json<ChatLogRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query(
        "INSERT INTO chat_logs (user_question, bot_answer) VALUES ($1, $2)"
    )
    .bind(payload.question)
    .bind(payload.answer)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {}", e))),
    }
}

// --- API 2: AMBIL STATISTIK (Dipanggil Admin Dashboard) ---
pub async fn get_chat_stats(
    State(state): State<AppState>,
) -> Result<Json<ChatStats>, (StatusCode, String)> {
    // Hitung total baris di tabel chat_logs
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM chat_logs")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {}", e)))?;

    // Return data statistik
    Ok(Json(ChatStats {
        total_chats: count,
        total_questions: count,
    }))
}