# 🦀 rust-capstone-backend

A simple **REST API** built with **Axum (Rust)** featuring basic request handlers, data parsing using extractors (`Path`, `Query`, `Json`, `State`), and custom error handling via `IntoResponse`.

Main framework: **Axum** — focused on ergonomics and modularity; fully integrated with `tower` and `tower-http`.  
📚 See Axum documentation at [docs.rs/axum](https://docs.rs/axum).

Database: **SQLite3** (planning migrate to **PostgreSQL**).

---

## ⚙️ Key Features

### 🧭 Routing & Extractors
- Declarative routing with `Router` (`GET`, `POST`, `PUT`, `DELETE`, etc.).
- Built-in extractors for parsing request data:
    - `Json<T>` → parses JSON body.
    - `Path<T>` → extracts parameters from the URL path.
    - `Query<T>` → parses query strings.
    - `State<T>` → accesses shared state (e.g. database pool, app configuration).

### 🗄️ Database Layer (SQLx)
Uses **[`sqlx`](https://docs.rs/sqlx)** as an *asynchronous database toolkit* to manage connections and execute SQL queries.

**Key features of `sqlx`:**
- Write raw SQL queries with **compile-time type checking**.
- Fully supports *async/await* (compatible with the `tokio` runtime).
- Automatically maps query results to Rust structs via the `FromRow` derive macro.
- Supports database transactions with `transaction()`.

Example:
```rust
let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", user_id)
    .fetch_one(&pool)
    .await?;
```

# 📊 Logging & Tracing
Integrated logging using the tracing and tracing-subscriber crates (in progress) for structured and contextual event logging.

# 🚀 Running the Application
```rust
# 1) Clone the repository
git clone https://github.com/ParesSensei/rust-capstone-be
cd rust-capstone-be

# 2) Run in development mode
cargo run
```

# 📚 Notes
- This project is developed as a Rust capstone project with focus on:
- Modular and maintainable project structure.
- Explicit error handling.
- Modern database integration using sqlx.
- Easy migration to PostgreSQL and observability via tracing.