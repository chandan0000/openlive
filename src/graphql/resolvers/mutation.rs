use async_graphql::{Context, Object, Result, InputObject};
use crate::graphql::types::user::UserGql;
use entity::users;
use sea_orm::{ActiveModelTrait, Set};
use uuid::Uuid;

#[derive(InputObject)]
pub struct CreateUserInput {
    pub full_name: Option<String>,
    pub email: String,
    pub phone_number: String,
    pub password: String, // in real app: hash this before storing
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<UserGql> {
        let db = ctx.data::<sea_orm::DatabaseConnection>()?;
        let new = users::ActiveModel {
            id: Set(Uuid::new_v4()),
            full_name: Set(input.full_name),
            email: Set(input.email.clone()),
            phone_number: Set(input.phone_number.clone()),
            password: Set(input.password.clone()),
            is_active: Set(Some(true)),
            ..Default::default()
        };
        let saved = new.insert(db).await?;
        Ok(saved.into())
    }
}
