use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use sqlx::PgPool;

use crate::{
    config::constants::BEARER,
    dto::{LoginInput, RegisterInput, TokenPayload},
    error::{ApiResult, Error},
    graphql::AppSchema,
    model::User,
    service::AuthService,
    utils::{jwt, validate_payload},
};

pub async fn authorize(user: User) -> Json<User> {
    Json(user)
}

pub async fn login(
    Json(input): Json<LoginInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<TokenPayload>> {
    validate_payload(&input)?;
    let user = AuthService::sign_in(input, &pool)
        .await
        .map_err(|_| Error::WrongCredentials)?;
    let token = jwt::sign(user.id)?;
    Ok(Json(TokenPayload {
        access_token: token,
        token_type: BEARER.to_string(),
    }))
}

pub async fn register(
    Json(input): Json<RegisterInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<(StatusCode, Json<TokenPayload>)> {
    validate_payload(&input)?;
    let user = AuthService::sign_up(input, &pool).await?;
    let token = jwt::sign(user.id)?;
    Ok((
        StatusCode::CREATED,
        Json(TokenPayload {
            access_token: token,
            token_type: BEARER.to_string(),
        }),
    ))
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

pub async fn graphql(
    schema: Extension<AppSchema>,
    req: GraphQLRequest,
    user: Option<User>,
) -> GraphQLResponse {
    schema.execute(req.into_inner().data(user)).await.into()
}
