use axum::{Router, routing::get};

use crate::router::{hello_server::hello_server, AppState};

pub   fn create_route_v1() -> Router<AppState> {
    Router::<AppState>::new().route("/v1/hello", get(hello_server))
}
