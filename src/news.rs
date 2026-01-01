use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::app_state::AppState;

// Model Database
#[derive(Debug, Serialize, FromRow)]
pub struct NewsItem {
    pub id: i32,
    pub title: String,
    pub category: String,
    pub image_url: String,
    pub content: String,
    pub date: String,
    pub read_minutes: i32,
}

// Model Input dari Admin
#[derive(Debug, Deserialize)]
pub struct NewsPayload {
    pub title: String,
    pub category: String,
    pub image_url: String,
    pub content: String,
    pub date: String,
    pub read_minutes: i32,
}

// 1. GET ALL NEWS (Untuk Halaman Home & Admin)
pub async fn get_all_news(
    State(state): State<AppState>,
) -> Result<Json<Vec<NewsItem>>, (StatusCode, String)> {
    let rows = sqlx::query_as::<_, NewsItem>("SELECT * FROM news ORDER BY id DESC")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {e}")))?;

    Ok(Json(rows))
}

// 2. ADD NEWS (Untuk Admin)
pub async fn add_news(
    State(state): State<AppState>,
    Json(payload): Json<NewsPayload>,
) -> Result<(StatusCode, Json<NewsItem>), (StatusCode, String)> {
    let inserted = sqlx::query_as::<_, NewsItem>(
        r#"
        INSERT INTO news (title, category, image_url, content, date, read_minutes)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#
    )
    .bind(payload.title)
    .bind(payload.category)
    .bind(payload.image_url)
    .bind(payload.content)
    .bind(payload.date)
    .bind(payload.read_minutes)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {e}")))?;

    Ok((StatusCode::CREATED, Json(inserted)))
}

// 3. DELETE NEWS (Untuk Admin)
pub async fn delete_news(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM news WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {e}")))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "News ID not found".to_string()));
    }

    Ok(StatusCode::OK)
}