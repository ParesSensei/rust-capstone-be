use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::app_state::AppState;

#[derive(Debug, Serialize, FromRow)]
pub struct TempatNongkrong {
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

#[derive(Debug, Deserialize)]
pub struct TempatNongkrongPayload {
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

pub async fn get_tempat_nongkrong(
    State(state): State<AppState>,
) -> Result<Json<Vec<TempatNongkrong>>, (StatusCode, String)> {
    let rows = sqlx::query_as::<_, TempatNongkrong>(
        r#"SELECT id, nama_tempat, kategori, alamat, jam_buka, jam_tutup, htm, link_gmaps, link_foto, deskripsi FROM tempat_nongkrong ORDER BY id"#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {e:?}")))?;

    Ok(Json(rows))
}

pub async fn get_tempat_nongkrong_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<TempatNongkrong>, (StatusCode, String)> {
    let row = sqlx::query_as::<_, TempatNongkrong>(
        r#"SELECT id, nama_tempat, kategori, alamat, jam_buka, jam_tutup, htm, link_gmaps, link_foto, deskripsi FROM tempat_nongkrong WHERE id = $1"#,
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::NOT_FOUND, format!("Not found / DB error: {e:?}")))?;

    Ok(Json(row))
}

pub async fn create_tempat_nongkrong(
    State(state): State<AppState>,
    Json(payload): Json<TempatNongkrongPayload>,
) -> Result<(StatusCode, Json<TempatNongkrong>), (StatusCode, String)> {
    let inserted = sqlx::query_as::<_, TempatNongkrong>(
        r#"
        INSERT INTO tempat_nongkrong (nama_tempat, kategori, alamat, jam_buka, jam_tutup, htm, link_gmaps, link_foto, deskripsi)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
        RETURNING id, nama_tempat, kategori, alamat, jam_buka, jam_tutup, htm, link_gmaps, link_foto, deskripsi
        "#,
    )
    .bind(payload.nama_tempat)
    .bind(payload.kategori)
    .bind(payload.alamat)
    .bind(payload.jam_buka)
    .bind(payload.jam_tutup)
    .bind(payload.htm)
    .bind(payload.link_gmaps)
    .bind(payload.link_foto)
    .bind(payload.deskripsi)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {e:?}")))?;

    Ok((StatusCode::CREATED, Json(inserted)))
}

pub async fn update_tempat_nongkrong(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<TempatNongkrongPayload>,
) -> Result<Json<String>, (StatusCode, String)> {
    let result = sqlx::query(
        r#"
        UPDATE tempat_nongkrong 
        SET nama_tempat=$1, kategori=$2, alamat=$3, jam_buka=$4, jam_tutup=$5, htm=$6, link_gmaps=$7, link_foto=$8, deskripsi=$9 
        WHERE id=$10
        "#
    )
    .bind(payload.nama_tempat)
    .bind(payload.kategori)
    .bind(payload.alamat)
    .bind(payload.jam_buka)
    .bind(payload.jam_tutup)
    .bind(payload.htm)
    .bind(payload.link_gmaps)
    .bind(payload.link_foto)
    .bind(payload.deskripsi)
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Update Error: {e:?}")))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "ID tidak ditemukan".to_string()));
    }

    Ok(Json("Update Berhasil".to_string()))
}

pub async fn delete_tempat_nongkrong(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<String>, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM tempat_nongkrong WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Delete Error: {e:?}")))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "ID tidak ditemukan".to_string()));
    }

    Ok(Json("Delete Berhasil".to_string()))
}