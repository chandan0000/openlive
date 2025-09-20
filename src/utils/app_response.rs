use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct APIResponse {
    pub status: u16,
    pub message: String,
    pub data: Value, // dynamic data type
}

impl APIResponse {
    pub fn new(status: StatusCode, message: impl Into<String>, data: Value) -> Self {
        Self {
            status: status.as_u16(),
            message: message.into(),
            data,
        }
    }
}

impl IntoResponse for APIResponse {
    fn into_response(self) -> axum::response::Response {
        let body = match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        (
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            body,
        )
            .into_response()
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
    pub error: String,
}

impl ErrorResponse {
    pub fn new(status: StatusCode, message: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            status: status.as_u16(),
            message: message.into(),
            error: error.into(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let body = match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        (
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            body,
        )
            .into_response()
    }
}
