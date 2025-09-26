use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;
pub mod user_route;
use crate::handler::auth_handler;
use crate::{router::hello_server::health, utils::app_state::AppState};

pub fn create_route_v1(app_state: AppState) -> Router<AppState> {
    Router::<AppState>::new()
        .route("/hello", get(health))
        .nest("/user", user_route::user_create_route(app_state))
        .route("/auth/signup", post(auth_handler::user_sign_up))
        .route("/auth/login", post(auth_handler::login))
}

