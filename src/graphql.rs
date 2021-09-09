use async_graphql::{Context, EmptySubscription, Result, Schema};

use crate::{
    dto::{AuthPayload, LoginInput, RegisterInput},
    error::Error,
    model::User,
    service::AuthService,
    utils::jwt,
};

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn current_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        Ok(ctx.data::<Option<User>>()?.to_owned())
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn login_user(&self, ctx: &Context<'_>, input: LoginInput) -> Result<AuthPayload> {
        let pool = utils::get_pg_pool(ctx)?;
        let user = AuthService::sign_in(input, &pool)
            .await
            .map_err(|_| Error::WrongCredentials)?;
        let token = jwt::sign(user.id)?;
        Ok(AuthPayload { token, user })
    }

    async fn register_user(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<AuthPayload> {
        let pool = utils::get_pg_pool(ctx)?;
        let user = AuthService::sign_up(input, &pool).await?;
        let token = jwt::sign(user.id)?;
        Ok(AuthPayload { token, user })
    }
}

mod utils {
    use async_graphql::{Context, Result};
    use sqlx::PgPool;

    pub fn get_pg_pool(ctx: &Context<'_>) -> Result<PgPool> {
        ctx.data::<PgPool>().map(|pool| pool.to_owned())
    }
}
