use axum::{
    http::Method,
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

// --- DAFTAR MODUL ---
mod admin;
mod app_state;
mod kuliner;
mod tempat_nongkrong;
mod user;
mod wisata_alam;
mod wisata_pendidikan;
mod chatbot; // Modul Chatbot
mod news;    // Modul Berita (BARU)

use crate::app_state::AppState;

// ADMIN + USER HANDLERS
use crate::admin::{admin_login_handler, admin_register_handler};
use crate::user::{login_user, register_user};

// WISATA ALAM HANDLERS
use crate::wisata_alam::{
    create_wisata, delete_wisata_alam, get_wisata_alam, get_wisata_alam_by_id, update_wisata_alam,
};

// WISATA PENDIDIKAN HANDLERS
use crate::wisata_pendidikan::{
    create_wisata_pendidikan, get_wisata_pendidikan, get_wisata_pendidikan_by_id,
};

// KULINER HANDLERS
use crate::kuliner::{
    create_kuliner, delete_kuliner, get_kuliner, get_kuliner_id, update_kuliner,
};

// TEMPAT NONGKRONG HANDLERS
use crate::tempat_nongkrong::{
    create_tempat_nongkrong, delete_tempat_nongkrong, get_tempat_nongkrong, get_tempat_nongkrong_id,
    update_tempat_nongkrong,
};

// CHATBOT HANDLERS
use crate::chatbot::{save_chat_log, get_chat_stats};

// NEWS HANDLERS (BARU)
use crate::news::{get_all_news, add_news, delete_news};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let pool = sqlx::postgres::PgPool::connect(&db_url)
        .await
        .expect("Failed to create postgre database pool");

    let state = AppState { pool };

    // Konfigurasi CORS (Izinkan PUT & DELETE untuk Admin Panel)
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        // ===== AUTH USER =====
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        
        // ===== AUTH ADMIN =====
        .route("/admin_register", post(admin_register_handler))
        .route("/admin_login", post(admin_login_handler))
        
        // ===== WISATA ALAM =====
        .route("/wisata_alam", get(get_wisata_alam))
        .route("/wisata_alam/{id}", get(get_wisata_alam_by_id))
        .route("/api/add_wisata", post(create_wisata))
        .route("/add_wisata", post(create_wisata)) // Fallback legacy
        .route("/api/update_wisata/{id}", put(update_wisata_alam))
        .route("/api/delete_wisata/{id}", delete(delete_wisata_alam))

        // ===== WISATA PENDIDIKAN =====
        .route("/wisata_pendidikan", get(get_wisata_pendidikan))
        .route("/wisata_pendidikan/{id}", get(get_wisata_pendidikan_by_id))
        .route("/add_wisata_pendidikan", post(create_wisata_pendidikan))

        // ===== KULINER =====
        .route("/kuliner", get(get_kuliner))
        .route("/kuliner/{id}", get(get_kuliner_id))
        .route("/get_kuliner", get(get_kuliner)) // alias
        .route("/api/add_kuliner", post(create_kuliner))
        .route("/add_kuliner", post(create_kuliner)) // Fallback legacy
        .route("/api/update_kuliner/{id}", put(update_kuliner))
        .route("/api/delete_kuliner/{id}", delete(delete_kuliner))

        // ===== TEMPAT NONGKRONG (CAFE) =====
        .route("/tempat_nongkrong", get(get_tempat_nongkrong))
        .route("/tempat_nongkrong/{id}", get(get_tempat_nongkrong_id))
        .route("/api/add_tempat_nongkrong", post(create_tempat_nongkrong))
        .route("/add_tempat_nongkrong", post(create_tempat_nongkrong)) // Fallback legacy
        .route("/api/update_cafe/{id}", put(update_tempat_nongkrong))
        .route("/api/delete_cafe/{id}", delete(delete_tempat_nongkrong))

        // ===== CHATBOT =====
        .route("/api/chat/log", post(save_chat_log))   // Lapor chat user
        .route("/api/chat/stats", get(get_chat_stats)) // Baca statistik chat

        // ===== NEWS / BERITA (BARU) =====
        .route("/api/news", get(get_all_news).post(add_news)) // GET semua & POST baru
        .route("/api/news/{id}", delete(delete_news))         // Hapus berita

        .with_state(state)
        .layer(cors);

    println!("Running server on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}