use async_graphql::{ComplexObject, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::graphql::{scalars::uuid::UUID, types::user::UserGql};

#[derive(SimpleObject, Clone, Debug)]
#[graphql(complex)]
pub struct PostType {
    pub id: UUID,
    pub title: String,
    pub text: String,
    pub likes: i32,
    pub dislikes: i32,
    pub views: i32,
    pub user_id: UUID,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_deleted: Option<bool>,
}

impl From<entity::post::Model> for PostType {
    fn from(model: entity::post::Model) -> Self {
        Self {
            id: UUID(model.id),
            title: model.title,
            text: model.text,
            likes: model.likes,
            dislikes: model.dislikes,
            views: model.views,
            user_id: UUID(model.user_id),
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
            deleted_at: model.deleted_at.map(|c| c.into()),
            is_deleted: model.is_deleted,
        }
    }
}

#[ComplexObject]
impl PostType {
    // relation: Post -> User
    async fn user(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<UserGql> {
        let db = ctx.data::<sea_orm::DatabaseConnection>()?;
        let user = entity::users::Entity::find_by_id(self.user_id.0)
            .one(db)
            .await?
            .ok_or("User not found")?;

        Ok(UserGql {
            id: UUID(user.id),
            full_name: user.full_name,
            email: user.email,
            phone_number: user.phone_number,
            profile_url: user.profile_url,
            is_active: user.is_active,
            is_verified: user.is_verified,
            created_at: user.created_at.map(|c| c.into()),
            updated_at: user.updated_at.map(|c| c.into()),
        })
    }
}

#[derive(InputObject)]
pub struct CreatePostInput {
    pub title: String,
    pub text: String,
    pub user_id: String,
}

#[derive(InputObject)]
pub struct UpdatePostInput {
    pub id: String,
    pub title: Option<String>,
    pub text: Option<String>,
}
