use crate::app_state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct WisataSql {
    name: String,
    category: String,
    address: String,
    open: String,
    close: String,
    htm: i32,
    gmaps: String,
    pictures: String,
}

#[derive(Serialize)]
pub struct WisataResponse {
    pub message: String,
}

#[debug_handler]
pub async fn create_wisata(
    State(state): State<AppState>,
    Json(payload): Json<WisataSql>,
) -> impl IntoResponse {
    let result = sqlx::query(
        "insert into wisata_alam(nama_tempat, kategori, alamat, jam_buka, jam_tutup, htm, link_gmaps, link_foto)
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
            Json(WisataResponse {
                message: "Wisata created".to_string(),
            }),
        ),

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(WisataResponse {
                message: format!("erorr: {}", e),
            }),
        ),
    }
}
