use axum::{Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthData {
    status: String,
}

pub async fn health() -> Json<HealthData> {
    let health_data = HealthData { status: "ok".into()};
    Json(health_data)
}