## Day 1: Basic Server Running

### Goal: Get a minimal Axum server running with one text route and one JSON route.

What I built today

Setup Axum project with Tokio runtime.

Wrote everything inside main() to stay fast and avoid early abstraction.

## Added two routes:

GET / returns static text.

GET /health returns JSON { "status": "ok" }.

## Current code entrypoint

Server listens on 127.0.0.1:3000.

Using tokio::net::TcpListener and axum::serve.

## Commands

```bash
cargo run
curl http://127.0.0.1:3000/
curl http://127.0.0.1:3000/health
```




## Day 2: Database Setup (PostgreSQL + SQLX)

**Goal:** Connect Axum API to a real PostgreSQL database and prepare migrations.

### What I did today

- Added `sqlx` with Postgres + UUID features  
- Installed SQLX CLI  
- Created a local PostgreSQL instance using Docker Compose  
- Created a new database: `axum_starter`  
- Added `.env` with `DATABASE_URL`  
- Generated the first migration:
```
sqlx migrate add create_users
```
- Wrote migration for `users` table:
```
CREATE TABLE users (
id UUID PRIMARY KEY,
email TEXT NOT NULL UNIQUE,
password_hash TEXT NOT NULL,
created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```
- Applied migrations using:
```
sqlx migrate run
```

### Current Status

- PostgreSQL container running  
- SQLX migrations working  
- App starts and connects to the DB  
- `/health` route now checks database connectivity  


## Day 3 — User Registration Implemented

Today’s goal: implement a real registration flow using SQLX, Argon2 password hashing, and proper request/response structs.

### Added Features

1. **POST /auth/register** endpoint
2. **Argon2 password hashing**
3. **UUID user IDs**
4. **Insert user into PostgreSQL**
5. **Structured request + response payloads**

### What Happens When You Register

* Client submits email and raw password
* Password is hashed using Argon2 + random salt
* A new UUID is generated
* User is inserted into the `users` table
* API returns the new user’s ID and email

### Example Request

```bash
curl -X POST http://127.0.0.1:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"secret123"}'
```

### Example Response

```json
{
    "id":"f8f69103-d517-4f66-b144-1ee1276da6eb",
    "email":"test@example.com"
}
```

### Files Added Today

* `src/routes/auth.rs`
* `argon2` and `rand` dependencies in Cargo.toml
* new `POST /auth/register` route in `routes/mod.rs`

### DB Schema Used

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```


