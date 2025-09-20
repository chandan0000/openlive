mod hello_server;
mod v1;
use crate::middleware::auth_middleware::auth;
use axum::{extract::FromRef, middleware, routing::get, Router};
use hello_server::health;
use sea_orm::DatabaseConnection;
use v1::create_route_v1;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection,
}

pub fn create_route(db: DatabaseConnection) -> Router {
    let app_state = AppState { database: db };
    Router::<AppState>::new()
        .route("/health", get(health))
        .nest("/api/v1", create_route_v1())
        .with_state(app_state)
        .layer(middleware::from_fn(auth))
}
