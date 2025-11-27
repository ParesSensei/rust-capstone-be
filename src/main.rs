use axum::{
    Router,
    routing::{get, post},
};

mod app_state;
mod user;
mod admin;

use crate::app_state::AppState;
use user::{login_user, register_user};
use admin::admin_register_handler;
use admin::admin_login_handler;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let pool = SqlitePool::connect(&db_url)
        .await
        .expect("Failed to create Sqlite database pool");

    let state = AppState { pool };

    let app = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/admin_register", post(admin_register_handler))
        .route("/admin_login", post(admin_login_handler))
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

        connection.close().await.expect("Unable to close DB connection");
        Ok(())
    }
}