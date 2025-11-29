use axum::{Router, routing::get};

mod health;
pub use health::health;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
}

async fn root() -> &'static str {
    "Welcome to the API written in Rust"
}