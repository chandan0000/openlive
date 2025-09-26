mod hello_server;
mod v1;
use std::sync::Arc;

use crate::{
    graphql::{
        mount::{graphql_handler, graphql_playground},
        schema::build_schema,
    },
    middleware::auth_middleware::auth_middlewarefn,
};
use axum::{
    Router,
    extract::State,
    middleware,
    routing::{get, post},
};
use hello_server::health;
use sea_orm::DatabaseConnection;
use v1::create_route_v1;

pub fn create_route(db: DatabaseConnection) -> Router {
    let schema = build_schema(db.clone());
    let app_state = crate::utils::app_state::AppState {
        database: Arc::new(db),
        gql_schema: schema.clone(),
    };
    Router::<crate::utils::app_state::AppState>::new()
        .route("/health", get(health))
        .nest("/api/v1", create_route_v1(app_state.clone()))
        .route("/api/graphql", post(graphql_handler))
        .route("/", get(graphql_playground))
        .with_state(app_state)
}
