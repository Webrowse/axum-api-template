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


## Day 3: User Registration Implemented

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

## Day 4: Login and JWT Authentication Implemented

## Overview
Today's work added full user login capability and secure token-based authentication. The backend can now verify user credentials, generate JWT tokens, and protect routes using a custom authentication middleware.

## What Was Implemented

### 1. Login Endpoint
A `/auth/login` route now:
- Accepts email and password
- Fetches the user record from the database
- Verifies the password using Argon2
- Generates a signed JWT token valid for 24 hours
- Returns the token to the client

### 2. JWT Token Generation
JWT contains:
- `sub`: user ID
- `iat`: issued at timestamp
- `exp`: expiration timestamp (24 hours)

Secret key loaded from environment via `JWT_SECRET`.

### 3. JWT Middleware
A reusable middleware validates:
- Authorization header format
- Token signature
- Token expiry
- Extracts user ID and attaches it to request extensions for downstream handlers

This middleware protects any route group using:
```
.layer(middleware::from_fn(require_auth))
```

### 4. Protected Route Example
Added a sample route:
- `GET /api/me`
- Requires a valid JWT
- Returns the user’s UUID extracted from the token

This confirms that the middleware and token verification pipeline are functioning.

### 5. Environment Requirements
`.env` must contain:
```
JWT_SECRET=your_long_random_secret
```
### 6. How to Test
Register a user:
```
POST /auth/register
```
Login to receive token:
```
POST /auth/login
```
Call protected route with the token:
```
GET /api/me
Authorization: Bearer <token>
```

## Status
Authentication layer is now complete and stable. System supports secure login, token issuance, and guarded routes.

## Day 5: Task Management CRUD

**Goal:** Implement complete task management system with database persistence.

### What I Built

- Created `tasks` table with foreign key to users (CASCADE delete)
- Organized task code into modular structure: dto, model, queries, routes
- Implemented full CRUD operations:
  - `POST /api/task` - create task
  - `GET /api/task` - list all user tasks
  - `PUT /api/task/{id}` - update task (partial updates with COALESCE)
  - `DELETE /api/task/{id}` - delete task
- All routes protected by JWT middleware
- User-scoped operations (users only access their own tasks)

### Files Added
- `migrations/20251130132950_create_tasks.sql`
- `src/routes/tasks/` module (dto, model, queries, routes)

---

