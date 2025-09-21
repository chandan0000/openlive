use async_graphql::{dataloader::Loader, Error};
use uuid::Uuid;
use std::collections::HashMap;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub struct UserLoader {
    pub db: DatabaseConnection,
}

impl Loader<Uuid> for UserLoader {
    type Value = entity::users::Model;
    type Error = Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let users = entity::users::Entity::find()
            .filter(entity::users::Column::Id.is_in(keys.to_vec()))
            .all(&self.db)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(users.into_iter().map(|u| (u.id, u)).collect())
    }
}
