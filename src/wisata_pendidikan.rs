use crate::app_state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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
    deskripsi: String, // Tambahan
}

#[derive(Serialize, FromRow)]
pub struct WisataResponseModel {
    pub id: i32,
    pub nama_tempat: String,
    pub kategori: String,
    pub alamat: String,
    pub jam_buka: String,
    pub jam_tutup: String,
    pub htm: i32,
    pub link_gmaps: String,
    pub link_foto: String,
    pub deskripsi: String, // Tambahan
}

#[derive(Serialize)]
pub struct WisataResponse {
    pub message: String,
}

#[debug_handler]
pub async fn create_wisata_pendidikan(
    State(state): State<AppState>,
    Json(payload): Json<WisataSql>,
) -> impl IntoResponse {
    let result = sqlx::query(
        "insert into wisata_pendidikan(nama_tempat, kategori, alamat, jam_buka, jam_tutup, htm, link_gmaps, link_foto, deskripsi)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
        .bind(&payload.name)
        .bind(&payload.category)
        .bind(&payload.address)
        .bind(&payload.open)
        .bind(&payload.close)
        .bind(&payload.htm)
        .bind(&payload.gmaps)
        .bind(&payload.pictures)
        .bind(&payload.deskripsi)
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(WisataResponse { message: "Wisata created".to_string() })),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(WisataResponse { message: format!("error: {}", e) })),
    }
}

#[debug_handler]
pub async fn get_wisata_pendidikan(State(state): State<AppState>) -> impl IntoResponse {
    let result = sqlx::query_as::<_, WisataResponseModel>("select * from wisata_pendidikan ORDER BY id")
        .fetch_all(&state.pool)
        .await;

    match result {
        Ok(data) => Json(data).into_response(),
        Err(err) => {
            eprintln!("Db error get_wisata_pendidikan: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_wisata_pendidikan_by_id(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let result = sqlx::query_as::<_, WisataResponseModel>(
        "SELECT * FROM wisata_pendidikan WHERE id = $1"
    ).bind(id).fetch_optional(&state.pool).await;

    match result {
        Ok(Some(data)) => Json(data).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Not found").into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {:?}", err)).into_response()
    }
}

pub async fn update_wisata_pendidikan(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<WisataSql>,
) -> impl IntoResponse {
    let result = sqlx::query(
        r#"UPDATE wisata_pendidikan 
           SET nama_tempat=$1, kategori=$2, alamat=$3, jam_buka=$4, jam_tutup=$5, htm=$6, link_gmaps=$7, link_foto=$8, deskripsi=$9 
           WHERE id=$10"#
    )
    .bind(&payload.name)
    .bind(&payload.category)
    .bind(&payload.address)
    .bind(&payload.open)
    .bind(&payload.close)
    .bind(&payload.htm)
    .bind(&payload.gmaps)
    .bind(&payload.pictures)
    .bind(&payload.deskripsi)
    .bind(id)
    .execute(&state.pool)
    .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                (StatusCode::NOT_FOUND, Json(WisataResponse { message: "ID Not Found".to_string() }))
            } else {
                (StatusCode::OK, Json(WisataResponse { message: "Updated successfully".to_string() }))
            }
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(WisataResponse { message: format!("Error: {}", e) })),
    }
}

pub async fn delete_wisata_pendidikan(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM wisata_pendidikan WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                (StatusCode::NOT_FOUND, Json(WisataResponse { message: "ID Not Found".to_string() }))
            } else {
                (StatusCode::OK, Json(WisataResponse { message: "Deleted successfully".to_string() }))
            }
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(WisataResponse { message: format!("Error: {}", e) })),
    }
}