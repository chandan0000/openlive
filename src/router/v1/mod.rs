use axum::{Router, routing::get};

use crate::router::hello_server;

pub   fn create_route_v1() -> Router {
    Router::new().route("/v1/hello", get(hello_server))
}
