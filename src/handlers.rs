use axum::{extract::Extension, http::StatusCode, Json};
use chrono::Utc;
use sqlx::PgPool;
use validator::Validate;

use crate::{
    error::{ApiResult, Error},
    model::{CreateUserData, User},
    utils::{encryption, jwt, validate_payload},
};

pub async fn authorize(user: User) -> Json<User> {
    Json(user)
}

lazy_static! {
    pub static ref BEARER: &'static str = "Bearer";
}

#[derive(Debug, Serialize)]
pub struct TokenPayload {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginInput {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

pub async fn login(
    Json(input): Json<LoginInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<TokenPayload>> {
    validate_payload(&input)?;
    let user = User::find_by_email(&input.email, &pool)
        .await
        .map_err(|_| Error::WrongCredentials)?;
    if encryption::verify_password(input.password, user.password).await? {
        let token = jwt::sign(user.id)?;
        Ok(Json(TokenPayload {
            access_token: token,
            token_type: BEARER.to_string(),
        }))
    } else {
        Err(Error::WrongCredentials.into())
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterInput {
    #[validate(length(min = 4, max = 10))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

pub async fn register(
    Json(input): Json<RegisterInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<(StatusCode, Json<TokenPayload>)> {
    validate_payload(&input)?;
    if User::find_by_email(&input.email, &pool).await.is_ok() {
        return Err(Error::DuplicateUserEmail.into());
    }
    if User::find_by_name(&input.name, &pool).await.is_ok() {
        return Err(Error::DuplicateUserName.into());
    }

    let data = CreateUserData {
        name: input.name,
        email: input.email,
        password: encryption::hash_password(input.password).await?,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let user = User::create(data, &pool).await?;
    let token = jwt::sign(user.id)?;
    Ok((
        StatusCode::CREATED,
        Json(TokenPayload {
            access_token: token,
            token_type: BEARER.to_string(),
        }),
    ))
}
