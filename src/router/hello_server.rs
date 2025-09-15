use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct HelloServer {
    hello: String,
}

pub async fn health() -> Json<HelloServer> {
    Json(HelloServer {
        hello: "Hey server".to_string(),
    })
}
