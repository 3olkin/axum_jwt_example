use axum::async_trait;

pub mod postgres;

#[async_trait]
pub trait DbPool {
    async fn retreive() -> Self;
}
