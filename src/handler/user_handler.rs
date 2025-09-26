use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use entity::users::Entity as Users;

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use axum::http::StatusCode;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, SqlErr};

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
    Path(user_id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let user_data = Users::find_by_id(user_id).one(&*db).await.unwrap();
    match user_data {
        Some(user) => APIResponse::new(
            StatusCode::OK,
            "User found",
            serde_json::json!({"user_info":UserResponse{
                full_name: user.full_name,
                email: user.email,
                phone_number: user.phone_number,
                profile_url: user.profile_url
            }}),
        )
        .into_response(),
        None => ErrorResponse::new(
            StatusCode::NOT_FOUND,
            "User not found",
            "No account with this email",
        )
        .into_response(),
    }
}