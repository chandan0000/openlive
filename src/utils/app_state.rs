use std::sync::Arc;

use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

use crate::graphql::schema::AppSchema;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: Arc<DatabaseConnection>,
    pub gql_schema: AppSchema,
}
