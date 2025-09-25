use crate::graphql::resolvers::query::QueryRoot;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use sea_orm::DatabaseConnection;

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema(db: DatabaseConnection) -> AppSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(db)
        .finish()
}
