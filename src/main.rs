use axum::{Json, Router, routing::get};
use serde::Serialize;

#[tokio::main]
async fn main(){
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to this API"
}

#[derive(Serialize)]
pub struct HealthData {
    name: String,
    age: u8,
    over_sixty: bool,
}

async fn health() -> Json<HealthData> {
    let hd = HealthData {
        name: String::from("Adarsh"),
        age: 12,
        over_sixty: false,
    };

    Json(hd)
}