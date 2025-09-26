use std::sync::Arc;
use axum::{Router, middleware, routing::get};
use sea_orm::DatabaseConnection;

use crate::{
    handler::user_handler,
    middleware::auth_middleware::auth_middlewarefn,
    utils::app_state::AppState,
};

pub fn user_create_route(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/profile", get(user_handler::user_profile_get))
        // ✅ pass the AppState clone AND the fn
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middlewarefn))
        .with_state(app_state) // give this router the same state
}
