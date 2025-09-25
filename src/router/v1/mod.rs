use axum::{
    Router,
    routing::{get, post},
};
pub mod user_route;
use crate::handler::auth_handler;
use crate::{router::hello_server::health, utils::app_state::AppState};

pub fn create_route_v1() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/hello", get(health))
        .nest("/user", user_route::user_create_route())
        .route("/auth/signup", post(auth_handler::user_sign_up))
        .route("/auth/login", post(auth_handler::login))
}
