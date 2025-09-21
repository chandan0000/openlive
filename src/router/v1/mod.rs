use axum::{
    Router,
    routing::{get, post},
};
use crate::router::{AppState, hello_server::health};
use crate::handler::auth;

pub fn create_route_v1() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/hello", get(health))
        .route("/auth/signup", post(auth::user_sign_up))
        .route("/auth/login", post(auth::login))
}
