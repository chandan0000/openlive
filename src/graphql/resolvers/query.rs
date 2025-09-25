use crate::graphql::types::post::PostType;
use crate::graphql::types::user::UserGql;
use async_graphql::{Context, Object, Result};
use entity::post;
use entity::users;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // get all users (optionally limit)
    async fn users(&self, ctx: &Context<'_>, limit: Option<usize>) -> Result<Vec<UserGql>> {
        // schema data has DatabaseConnection injected
        let db = ctx.data::<sea_orm::DatabaseConnection>()?;
        let mut query = users::Entity::find();
        if let Some(l) = limit {
            query = query.limit(l as u64);
        }
        let models = query.all(db).await?;
        Ok(models.into_iter().map(Into::into).collect())
    }

    // get user by email
    async fn user_by_email(&self, ctx: &Context<'_>, email: String) -> Result<Option<UserGql>> {
        let db = ctx.data::<sea_orm::DatabaseConnection>()?;
        use users::Column;
        let u = users::Entity::find()
            .filter(Column::Email.eq(email))
            .one(db)
            .await?;
        Ok(u.map(Into::into))
    }
    // posts
    async fn posts(&self, ctx: &Context<'_>, limit: Option<usize>) -> Result<Vec<PostType>> {
        let db = ctx.data::<sea_orm::DatabaseConnection>()?;
        let mut query = post::Entity::find();

        if let Some(l) = limit {
            query = query.limit(l as u64);
        }

        let models = query.all(db).await?;
        Ok(models.into_iter().map(Into::into).collect())
    }
}
