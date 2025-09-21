use axum::response::IntoResponse;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use crate::graphql::schema::AppSchema;
use axum::extract::State;

pub async fn graphql_handler(
    State(schema): State<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    // GraphiQL / Playground UI (browser)
    axum::response::Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    ))
}
