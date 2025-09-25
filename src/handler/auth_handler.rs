use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::utils::{
    app_response::*,
    jwt_token::create_token,
    password::{hash, password_match},
};
use entity::users::{ActiveModel, Column as UserColumn, Entity as Users};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, SqlErr};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserSignUpRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub full_name: Option<String>,
    pub email: String,
    pub phone_number: String,
    pub profile_url: Option<String>,
}

pub async fn user_sign_up(
    State(db): State<Arc<DatabaseConnection>>,
    Json(req): Json<UserSignUpRequest>,
) -> impl IntoResponse {
    let hashed_password = hash(req.password.clone()).map_err(|err| err.into_response());

    let new_user = ActiveModel {
        full_name: Set(Some(req.name)),
        email: Set(req.email),
        password: Set(hashed_password.unwrap()),
        phone_number: Set(req.phone_number),
        ..Default::default()
    };

    let inserted_user = Users::insert(new_user)
        .exec_with_returning(&*db)
        .await
        .map_err(|err| {
            if let Some(SqlErr::UniqueConstraintViolation(_)) = err.sql_err() {
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
        });
    match inserted_user {
        Ok(user) => match create_token(&user.id.to_string()) {
            Ok(token) => APIResponse::new(
                StatusCode::CREATED,
                "Account create successfully",
                serde_json::json!({"jwt_token":token, "token_type":"Bearer", "user_info":UserResponse{
                    full_name: user.full_name,
                    email: user.email,
                    phone_number: user.phone_number,
                    profile_url: user.profile_url
                }}),
            )
            .into_response(),
            Err(err) => err.into_response(),
        },
        Err(err) => err.into_response(),
    }
}

#[derive(Deserialize)]
pub struct LogInRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(db): State<Arc<   DatabaseConnection>>,
    Json(userlogin_req): Json<LogInRequest>,
) -> impl IntoResponse {
    let user_data = Users::find()
        .filter(UserColumn::Email.eq(userlogin_req.email.clone()))
        .one(&*db)
        .await
        .unwrap();
    if let Some(u) = user_data {
        match password_match(&userlogin_req.password, &u.password) {
            Ok(true) => {
                // yaha hum token handle karenge
                match create_token(&u.id.to_string()) {
                    Ok(token) => APIResponse::new(
                        StatusCode::OK,
                        "Login successful",
                        serde_json::json!({
                            "jwt_token": token,
                            "token_type": "Bearer",
                            "user_info": {
                                "full_name": u.full_name,
                                "email": u.email,
                                "phone_number": u.phone_number,
                                "profile_url": u.profile_url,
                            }
                        }),
                    )
                    .into_response(),
                    Err(err) => err.into_response(), // token generation failed
                }
            }
            Ok(false) => ErrorResponse::new(
                StatusCode::UNAUTHORIZED,
                "Invalid credentials",
                "Password does not match",
            )
            .into_response(),
            Err(err) => err.into_response(), // password_match failed
        }
    } else {
        ErrorResponse::new(
            StatusCode::NOT_FOUND,
            "User not found",
            "No account with this email",
        )
        .into_response()
    }
}
