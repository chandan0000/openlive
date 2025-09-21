use async_graphql::dataloader::DataLoader;
use sea_orm::{sqlx::types::uuid, DatabaseConnection};

use crate::graphql::dataloaders::user_loader::UserLoader;

// #[derive(Clone)]
pub struct GqlContextState {
    pub db: DatabaseConnection,           
    pub current_user_id: Option<uuid::Uuid>,
    pub user_loader: DataLoader<UserLoader>,
}

impl GqlContextState {
    pub fn new(db: DatabaseConnection, current_user_id: Option<uuid::Uuid>) -> Self {
        let loader = DataLoader::new(UserLoader { db: db.clone() }, tokio::spawn);
        Self { db, current_user_id, user_loader: loader }
    }
}
