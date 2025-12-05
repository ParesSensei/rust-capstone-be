# 🦀 rust-capstone-backend

A simple **REST API** built with **Axum (Rust)** featuring basic request handlers, data parsing using extractors (`Path`, `Query`, `Json`, `State`), and custom error handling via `IntoResponse`.

Main framework: **Axum** — focused on ergonomics and modularity; fully integrated with `tower` and `tower-http`.  
📚 See Axum documentation at [docs.rs/axum](https://docs.rs/axum).

Database: **PostgreSQL**.

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
```bash
# 1) Clone the repository
git clone https://github.com/ParesSensei/rust-capstone-be
cd rust-capstone-be

# 2) Run in development mode
cargo run

# 3) Run database
postgres://username:password@host:port/exploremas
```


# API DOCS
documentation to use api.
1. route("/register", post(register_user))  
example: 
```json
{
    "username": "hayanambatukam",
    "password": "password",
    "email": "aha@gmail.com"
}
```
response:

2. route("/login", post(login_user))  
example: 
```json
{
  "username": "hayanambatukam",
  "password": "password"
}
```
response success:
```Logged in```  
response failed(wrong password):
```Login failed```  
3. route("/admin_register", post(admin_register_handler))  
example:
```json
{
  "username": "admin",
  "password": "admin",
  "email": "admin@gmail.com"
}
```
response:
```json
{
    "message": "Success create new admin"
}
```

4. route("/admin_login", post(admin_login_handler))  
example:
```json
{
  "username": "admin",
  "password": "admin"
}
```
response:
```Logged in```

5. route("/add_wisata", post(create_wisata))  
example:
```json
{
    "name": "Gunung Slamet",
    "category": "wisata alam",
    "address": "Sawah Dan Kebun, Gunungsari, Kec. Pulosari, Kabupaten Pemalang, Jawa Tengah",
    "open": "18:22",
    "close": "23:59:00",
    "htm": 35000,
    "gmaps": "https://maps.app.goo.gl/DomnNsbzzUa2oxcf7",
    "pictures": "https://jetex.id/blog/wp-content/uploads/2025/10/YouTube.jpg"
}
```
response:
```Wisata created```

6. route("/wisata_alam", get(get_wisata_alam))  
example:
```json
"http://localhost:3000/wisata_alam"
```
response:
```json
{
    "name": "Gunung Slamet",
    "category": "wisata alam",
    "address": "Sawah Dan Kebun, Gunungsari, Kec. Pulosari, Kabupaten Pemalang, Jawa Tengah",
    "open": "18:22",
    "close": "23:59:00",
    "htm": 35000,
    "gmaps": "https://maps.app.goo.gl/DomnNsbzzUa2oxcf7",
    "pictures": "https://jetex.id/blog/wp-content/uploads/2025/10/YouTube.jpg"
}
```

7. route("/wisata_alam/{id}", get(get_wisata_alam_by_id))  
example: 
```json
"http://localhost:3000/wisata_alam/1"
```
response:
```json
{
    "name": "Gunung Slamet",
    "category": "wisata alam",
    "address": "Sawah Dan Kebun, Gunungsari, Kec. Pulosari, Kabupaten Pemalang, Jawa Tengah",
    "open": "18:22",
    "close": "23:59:00",
    "htm": 35000,
    "gmaps": "https://maps.app.goo.gl/DomnNsbzzUa2oxcf7",
    "pictures": "https://jetex.id/blog/wp-content/uploads/2025/10/YouTube.jpg"
}
```
8. route("/add_wisata_pendidikan", post(create_wisata_pendidikan))  
example:
```json
{
    "name": "The forest island",
    "category": "wisata edukasi",
    "address": "Jl. Raya Baturaden No.Km. 6, Dusun I Pandak, Pandak, Kec. Baturaden, Kabupaten Banyumas, Jawa Tengah 53151",
    "open": "08:00",
    "close": "17:00",
    "htm": 7000,
    "gmaps": "https://maps.app.goo.gl/jbwkqV1ZnhSH3x3XA",
    "pictures": "https://jetex.id/blog/wp-content/uploads/2025/10/YouTube.jpg"
}
```
response: 
```Wisata created```

9. route("/wisata_pendidikan", get(get_wisata_pendidikan))  
example:
```json
"http://localhost:3000/wisata_pendidikan"
```
response:
```json
{
    "name": "The forest island",
    "category": "wisata edukasi",
    "address": "Jl. Raya Baturaden No.Km. 6, Dusun I Pandak, Pandak, Kec. Baturaden, Kabupaten Banyumas, Jawa Tengah 53151",
    "open": "08:00",
    "close": "17:00",
    "htm": 7000,
    "gmaps": "https://maps.app.goo.gl/jbwkqV1ZnhSH3x3XA",
    "pictures": "https://jetex.id/blog/wp-content/uploads/2025/10/YouTube.jpg"
}
```

10. route("/wisata_pendidikan/{id}", get(get_wisata_pendidikan_by_id))  
    example:
```json
"http://localhost:3000/wisata_pendidikan/1"
```
response:
```json
{
    "name": "The forest island",
    "category": "wisata edukasi",
    "address": "Jl. Raya Baturaden No.Km. 6, Dusun I Pandak, Pandak, Kec. Baturaden, Kabupaten Banyumas, Jawa Tengah 53151",
    "open": "08:00",
    "close": "17:00",
    "htm": 7000,
    "gmaps": "https://maps.app.goo.gl/jbwkqV1ZnhSH3x3XA",
    "pictures": "https://jetex.id/blog/wp-content/uploads/2025/10/YouTube.jpg"
}
```
.route("/kuliner", post(create_kuliner))
.route("/get_kuliner", get(get_kuliner))
.route("/kuliner/{id}", get(get_kuliner_id))

# 📚 Database migrations error
because of the database error, so currently we are creating a manual database without migration.
you can check in [schema_manual.txt](schema_manual.txt)

# 📚 Notes
- This project is developed as a Rust capstone project with focus on:
- Modular and maintainable project structure.
- Explicit error handling.
- Modern database integration using sqlx.
- Easy migration to PostgreSQL and observability via tracing.