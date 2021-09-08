use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::Result,
    model::{CreateUserData, User},
};

impl User {
    pub async fn find_by_uuid(uuid: Uuid, pool: &PgPool) -> Result<User> {
        let sql = format!("SELECT * FROM {} WHERE uuid = $1 LIMIT 1", User::TABLE);
        Ok(sqlx::query_as(&sql).bind(uuid).fetch_one(pool).await?)
    }

    pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<User> {
        let sql = format!("SELECT * FROM {} WHERE email = $1 LIMIT 1", User::TABLE);
        Ok(sqlx::query_as(&sql).bind(email).fetch_one(pool).await?)
    }

    pub async fn create(data: CreateUserData, pool: &PgPool) -> Result<User> {
        let sql = format!(
            "
            INSERT INTO {} (uuid, name, email, password, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            ",
            User::TABLE
        );
        Ok(sqlx::query_as(&sql)
            .bind(data.uuid)
            .bind(data.name)
            .bind(data.email)
            .bind(data.password)
            .bind(data.created_at)
            .bind(data.updated_at)
            .fetch_one(pool)
            .await?)
    }
}
