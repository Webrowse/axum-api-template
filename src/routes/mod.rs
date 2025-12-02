use axum::{
    Router,
    routing::{get, post},
    middleware,
    extract::Request,
};

mod health;
mod auth;
mod middleware_auth;

pub use health::health;
pub use auth::register;

use crate::state::AppState;
use crate::routes::auth::login;


pub fn routes() -> Router<AppState>{
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .nest (
        "/api",
        Router::new()
        .route("/me", get(me_handler))
        .layer(middleware::from_fn(middleware_auth::require_auth))
    )
}

async fn root() -> &'static str {
    "Welcome to the API written in Rust"
}

async fn me_handler(req: Request<axum::body::Body>) -> impl axum::response::IntoResponse {
    let user_id = req.extensions().get::<uuid::Uuid>().cloned();
    match user_id {
        Some(u) => (axum::http::StatusCode::OK, format!("user_id: {}", u)),
        None => (axum::http::StatusCode::UNAUTHORIZED, "no user".into()),
    }
}