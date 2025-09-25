mod hello_server;
mod v1;
use std::sync::Arc;

use crate::graphql::{
    mount::{graphql_handler, graphql_playground},
    schema::{AppSchema, build_schema},
};
use axum::{
    Router,
    extract::FromRef,
    routing::{get, get_service, post},
};
use hello_server::health;
use sea_orm::DatabaseConnection;
use tower_http::services::{ServeDir, ServeFile};
use v1::create_route_v1;


pub fn create_route(db: DatabaseConnection) -> Router {
    let schema = build_schema(db.clone());

    // create app state with db + schema
    let app_state = crate::utils::app_state::AppState {
        database: Arc::new(db),
        gql_schema: schema.clone(),
    };
    Router::<crate::utils::app_state::AppState>::new()
        .route("/health", get(health))
        .nest("/api/v1", create_route_v1())
        .route("/api/graphql", post(graphql_handler))
        .route("/", get(graphql_playground))
        .nest_service(
            "/admin",
            get_service(
                ServeDir::new("assets/admin")
                    .not_found_service(ServeFile::new("assets/admin/index.html")),
            ),
        )
        .with_state(app_state)
    // .layer(middleware::from_fn(auth))
}
