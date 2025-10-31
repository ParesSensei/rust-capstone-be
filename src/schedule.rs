use axum::{extract::State, response::Json};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};
use crate::user::AppState;

#[derive(Serialize, FromRow)]
pub struct Schedule {
    pub id: i64,
    pub koridor_id: i64,
    pub halte_id: i64,
    pub departure: String,
    pub day: String,
}

pub async fn view_schedule(State(state): State<AppState>) -> Json<Vec<Schedule>> {
    let schedules = sqlx::query_as::<_, Schedule>("SELECT * FROM jadwal")
        .fetch_all(&state.pool)
        .await
        .expect("Gagal ambil jadwal");

    Json(schedules)
}
