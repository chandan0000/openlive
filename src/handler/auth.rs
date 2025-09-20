use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::utils::app_response::*;
use entity::users::{ActiveModel, Entity as Users};
use sea_orm::{DatabaseConnection, EntityTrait, Set, SqlErr};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserSignUpRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
}

pub async fn user_sign_up(
    State(db): State<DatabaseConnection>,
    Json(req): Json<UserSignUpRequest>,
) -> impl IntoResponse {
    println!("User signed up: Name: {}, Email: {}", req.name, req.email);
    let new_user = ActiveModel {
        full_name: Set(Some(req.name)),
        email: Set(req.email),
        password: Set(req.password),
        phone_number: Set(req.phone_number),
        ..Default::default()
    };
    Users::insert(new_user).exec(&db).await.map_or_else(
        |_err| {
            if let Some(SqlErr::UniqueConstraintViolation(_)) = _err.sql_err() {
                return ErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    "User already exists",
                    "User already exists",
                )
                .into_response();
            }

            ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert user",
                "Database error",
            )
            .into_response()
        },
        |_ok| {
            APIResponse::new(
                StatusCode::CREATED,
                "User created successfully",
                serde_json::json!({}),
            )
            .into_response()
        },
    )
}
