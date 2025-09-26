use axum::{
    extract::{Request, State},
    response::IntoResponse,
};
use entity::users::{Entity as Users, Model as User};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use axum::http::StatusCode;
use serde::Serialize;
use crate::utils::app_response::{APIResponse, ErrorResponse};

#[derive(Serialize)]
pub struct UserResponse {
    pub full_name: Option<String>,
    pub email: String,
    pub phone_number: String,
    pub profile_url: Option<String>,
}

pub async fn user_profile_get(
    State(db): State<Arc<DatabaseConnection>>,
    request: Request,
) -> impl IntoResponse {
    // Extract user from request extensions (set by auth middleware)
    let user = match request.extensions().get::<User>() {
        Some(user) => user,
        None => {
            return ErrorResponse::new(
                StatusCode::UNAUTHORIZED,
                "User not authenticated",
                "Authentication required",
            )
            .into_response();
        }
    };

    // Return user profile
    APIResponse::new(
        StatusCode::OK,
        "User profile retrieved successfully",
        serde_json::json!({"user_info": UserResponse {
            full_name: user.full_name.clone(),
            email: user.email.clone(),
            phone_number: user.phone_number.clone(),
            profile_url: user.profile_url.clone()
        }}),
    )
    .into_response()
}
