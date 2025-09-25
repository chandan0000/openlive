use async_graphql::SimpleObject;
use entity::users;
use serde::{Deserialize, Serialize};

use crate::graphql::scalars::uuid::UUID;

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct UserGql {
    pub id: UUID,
    pub full_name: Option<String>,
    pub email: String,
    pub phone_number: String,
    pub profile_url: Option<String>,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<users::Model> for UserGql {
    fn from(m: users::Model) -> Self {
        UserGql {
            id: UUID(m.id),
            full_name: m.full_name,
            email: m.email,
            phone_number: m.phone_number,
            profile_url: m.profile_url,
            is_active: m.is_active,
            is_verified: m.is_verified,
            created_at: m.created_at.map(|d| d.into()), 
            updated_at: m.updated_at.map(|d| d.into()),
        }
    }
}
