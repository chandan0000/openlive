mod hello_server;
mod v1;
use crate::{
    graphql::{mount::{graphql_handler, graphql_playground}, schema::{build_schema, AppSchema}},
    middleware::auth_middleware::auth,
};
use axum::{
    Router,
    extract::FromRef,
    middleware,
    routing::{get, post},
};
use hello_server::health;
use sea_orm::DatabaseConnection;
use v1::create_route_v1;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub gql_schema: AppSchema,
}

pub fn create_route(db: DatabaseConnection) -> Router {
    let schema = build_schema(db.clone());

    // create app state with db + schema
    let app_state = crate::router::AppState {
        database: db.clone(),
        gql_schema: schema.clone(),
    };
    Router::<AppState>::new()
        .route("/health", get(health))
        .nest("/api/v1", create_route_v1())
        .route("/graphql", post(graphql_handler))
        .route("/", get(graphql_playground))
        .with_state(app_state)
        .layer(middleware::from_fn(auth))
}
