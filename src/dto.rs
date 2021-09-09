use validator::Validate;

use crate::model::User;

#[derive(Debug, Deserialize, Validate, InputObject)]
pub struct LoginInput {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, InputObject)]
pub struct RegisterInput {
    #[validate(length(min = 4, max = 10))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, SimpleObject)]
pub struct AuthPayload {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize)]
pub struct TokenPayload {
    pub access_token: String,
    pub token_type: String,
}
