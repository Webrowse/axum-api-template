use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use argon2::{Argon2, PasswordHasher};
use rand::rngs::OsRng;
// use rand_core::OsRng;
use argon2::password_hash::{SaltString, PasswordHash};

#[derive(Deserialize)]
pub struct RegistrationRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: Uuid,
    pub email: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegistrationRequest>
) -> Json<RegisterResponse> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();

    let password_hash = argon.hash_password(payload.password.as_bytes(), &salt).unwrap().to_string();
    let user_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO users (id, email, password_hash)
        VALUES ($1,$2,$3)
        "#,
        user_id, payload.email, password_hash
    )
    .execute(&state.db).await.unwrap();

    Json(RegisterResponse {
        id: user_id,
        email: payload.email,
    })
}