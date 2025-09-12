mod hello_server;
mod v1;
use v1::create_route_v1;
use axum::{Router, routing::get};
use hello_server::hello_server;

 pub fn create_route() -> Router {
    Router::new().route("/", get(hello_server)).merge(create_route_v1())
}
