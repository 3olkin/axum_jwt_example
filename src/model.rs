use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, SimpleObject)]
pub struct User {
    #[serde(skip_serializing)]
    #[graphql(skip)]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    #[graphql(skip)]
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub const TABLE: &'static str = "users";
}

#[derive(Debug)]
pub struct CreateUserData {
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
