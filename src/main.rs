use axum::{
    routing::{get, post},
    Router,
};

mod admin;
mod app_state;
mod user;
mod wisata_alam;
mod wisata_pendidikan;
mod kuliner;
mod tempat_nongkrong;

use crate::app_state::AppState;

// admin + user
use crate::admin::{admin_login_handler, admin_register_handler};
use crate::user::{login_user, register_user};

// wisata
use crate::wisata_alam::{create_wisata, get_wisata_alam, get_wisata_alam_by_id};
use crate::wisata_pendidikan::{
    create_wisata_pendidikan, get_wisata_pendidikan, get_wisata_pendidikan_by_id,
};

// kuliner + tempat nongkrong
use crate::kuliner::{create_kuliner, get_kuliner, get_kuliner_id};
use crate::tempat_nongkrong::{
    create_tempat_nongkrong, get_tempat_nongkrong, get_tempat_nongkrong_id,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let pool = sqlx::postgres::PgPool::connect(&db_url)
        .await
        .expect("Failed to create postgre database pool");

    let state = AppState { pool };

    let app = Router::new()
        // ===== auth user =====
        .route("/register", post(register_user))
        .route("/login", post(login_user))

        // ===== auth admin =====
        .route("/admin_register", post(admin_register_handler))
        .route("/admin_login", post(admin_login_handler))

        // ===== wisata alam =====
        .route("/wisata_alam", get(get_wisata_alam))
        .route("/wisata_alam/{id}", get(get_wisata_alam_by_id))
        .route("/add_wisata", post(create_wisata))

        // ===== wisata pendidikan =====
        .route("/wisata_pendidikan", get(get_wisata_pendidikan))
        .route("/wisata_pendidikan/{id}", get(get_wisata_pendidikan_by_id))
        .route("/add_wisata_pendidikan", post(create_wisata_pendidikan))

        // ===== kuliner =====
        // endpoint utama (disarankan untuk FE baru)
        .route("/kuliner", get(get_kuliner).post(create_kuliner))
        .route("/kuliner/{id}", get(get_kuliner_id))

        // alias biar FE lama tidak 404 (penting!)
        .route("/get_kuliner", get(get_kuliner))
        .route("/add_kuliner", post(create_kuliner))

        // ===== tempat nongkrong =====
        .route("/tempat_nongkrong", get(get_tempat_nongkrong))
        .route("/tempat_nongkrong/{id}", get(get_tempat_nongkrong_id))
        .route("/add_tempat_nongkrong", post(create_tempat_nongkrong))

        .with_state(state);

    println!("Running server on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
