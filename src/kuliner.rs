use crate::app_state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct KulinerSql {
    name: String,
    category: String,
    address: String,
    open: String,
    close: String,
    htm: i32,
    gmaps: String,
    pictures: String,
}

#[derive(Serialize, FromRow)]
pub struct KulinerResponseModel {
    nama_tempat: String,
    kategori: String,
    alamat: String,
    jam_buka: String,
    jam_tutup: String,
    htm: i32,
    link_gmaps: String,
    link_foto: String,
}

#[derive(Serialize)]
pub struct KulinerResponse {
    pub message: String,
}

#[debug_handler]
pub async fn create_kuliner(
    State(state): State<AppState>,
    Json(payload): Json<KulinerSql>,
) -> impl IntoResponse {
    let result = sqlx::query(
        "insert into kuliner(nama_tempat, kategori, alamat, jam_buka, jam_tutup, htm, link_gmaps, link_foto)
        values ($1, $2, $3, $4, $5, $6, $7, $8)")
        .bind(&payload.name)
        .bind(&payload.category)
        .bind(&payload.address)
        .bind(&payload.open)
        .bind(&payload.close)
        .bind(&payload.htm)
        .bind(&payload.gmaps)
        .bind(&payload.pictures)
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(KulinerResponse {
                message: "Kuliner created".to_string(),
            }),
        ),

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(KulinerResponse {
                message: format!("erorr: {}", e),
            }),
        ),
    }
}

#[debug_handler]
pub async fn get_kuliner(State(state): State<AppState>) -> impl IntoResponse {
    let result = sqlx::query_as::<_, KulinerResponseModel>("select * from kuliner")
        .fetch_one(&state.pool)
        .await;

    match result {
        Ok(data) => Json(data).into_response(),
        Err(err) => {
            eprintln!("Db error {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_kuliner_id(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let result = sqlx::query_as::<_, KulinerResponseModel>(
        "SELECT * FROM kuliner WHERE id = $1"
    ).bind(id).fetch_optional(&state.pool).await;

    match result {
        Ok(Some(data)) => Json(data).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Not found").into_response(),
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}