use crate::utils::jwt_token::decode_token;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse},
};
use entity::users::Entity as Users;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::sync::Arc;

pub async fn auth_middlewarefn(
    State(db): State<Arc<DatabaseConnection>>,
    req: Request,
    next: Next,
) -> impl IntoResponse {
    println!("Auth middleware request detected");

    // Extract authorization header
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            header.strip_prefix("Bearer ").unwrap_or("")
        }
        _ => {
            return crate::utils::app_response::ErrorResponse::new(
                StatusCode::UNAUTHORIZED,
                "Unauthorized",
                "No authorization header found",
            )
            .into_response();
        }
    };

    // Decode JWT token to get user ID
    let user_id = match decode_token(auth_header) {
        Ok(id) => id,
        Err(_) => {
            return crate::utils::app_response::ErrorResponse::new(
                StatusCode::UNAUTHORIZED,
                "Unauthorized",
                "Invalid authorization header",
            )
            .into_response();
        }
    };

    // Parse user ID to UUID
    let user_uuid = match uuid::Uuid::parse_str(&user_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return crate::utils::app_response::ErrorResponse::new(
                StatusCode::UNAUTHORIZED,
                "Unauthorized",
                "Invalid authorization header",
            )
            .into_response();
        }
    };

    // Find user in database
    let user = match Users::find_by_id(user_uuid).one(&*db).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return crate::utils::app_response::ErrorResponse::new(
                StatusCode::UNAUTHORIZED,
                "Unauthorized",
                "Invalid authorization header",
            )
            .into_response();
        }
        Err(_) => {
            return crate::utils::app_response::ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                "Something went wrong",
            )
            .into_response();
        }
    };

    // Insert user into request extensions so handlers can access it
    let mut req = req;
    req.extensions_mut().insert(user);

    next.run(req).await
}
