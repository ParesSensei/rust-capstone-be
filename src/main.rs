use axum::{routing::post, Router};
use axum::routing::get;

mod admin;
mod app_state;
mod user;
mod wisata_alam;
mod wisata_pendidikan;
mod kuliner;
mod tempat_nongkrong;

use crate::app_state::AppState;
use crate::wisata_alam::create_wisata;
use crate::admin::admin_login_handler;
use crate::admin::admin_register_handler;
use crate::user::{login_user, register_user};
use crate::wisata_alam::get_wisata_alam;
use crate::wisata_alam::get_wisata_alam_by_id;
use crate::wisata_pendidikan::get_wisata_pendidikan_by_id;
use crate::wisata_pendidikan::get_wisata_pendidikan;
use crate::wisata_pendidikan::create_wisata_pendidikan;
use crate::kuliner::create_kuliner;
use crate::kuliner::get_kuliner;
use crate::kuliner::get_kuliner_id;
use crate::tempat_nongkrong::get_tempat_nongkrong;
use crate::tempat_nongkrong::get_tempat_nongkrong_id;
use crate::tempat_nongkrong::create_tempat_nongkrong;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    // println!("DATABASE_URL = {:?}", std::env::var("DATABASE_URL"));

    let pool = sqlx::postgres::PgPool::connect(&db_url)
        .await
        .expect("Failed to create postgre database pool");

    // sqlx::migrate!()
    //     .run(&pool)
    //     .await
    //     .expect("Failed to run migrations");

    let state = AppState { pool };

    let app = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/admin_register", post(admin_register_handler))
        .route("/admin_login", post(admin_login_handler))
        .route("/add_wisata", post(create_wisata))
        .route("/wisata_alam", get(get_wisata_alam))
        .route("/wisata_alam/{id}", get(get_wisata_alam_by_id))
        .route("/add_wisata_pendidikan", post(create_wisata_pendidikan))
        .route("/wisata_pendidikan", get(get_wisata_pendidikan))
        .route("/wisata_pendidikan/{id}", get(get_wisata_pendidikan_by_id))
        .route("/kuliner", post(create_kuliner))
        .route("/get_kuliner", get(get_kuliner))
        .route("/kuliner/{id}", get(get_kuliner_id))
        .route("/tempat_nongkrong", get(get_tempat_nongkrong))
        .route("/add_tempat_nongkrong", post(create_tempat_nongkrong))
        .route("/tempat_nongkrong/{id}", get(get_tempat_nongkrong_id))
        .with_state(state);

    println!("Running server on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use sqlx::{Connection, Error, PgConnection};

    #[tokio::test]
    async fn test_manual_connection() -> Result<(), Error> {
        dotenvy::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        let connection: PgConnection = PgConnection::connect(db_url.as_str()).await?;

        connection
            .close()
            .await
            .expect("Unable to close DB connection");
        Ok(())
    }
}
