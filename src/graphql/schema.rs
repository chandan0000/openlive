use async_graphql::{EmptySubscription, Schema};
use sea_orm::DatabaseConnection;
use crate::graphql::resolvers::mutation::MutationRoot;
use crate::graphql::resolvers::query::QueryRoot;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema(db: DatabaseConnection) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish()
}
