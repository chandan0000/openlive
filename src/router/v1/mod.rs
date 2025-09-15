use axum::{Router, routing::get};

use crate::router::{hello_server::health, AppState};

pub   fn create_route_v1() -> Router<AppState> {
    Router::<AppState>::new().route("/hello", get(health))
}
