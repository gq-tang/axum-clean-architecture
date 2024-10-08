use crate::{
    api,
    domain::{
        error::{ApiError, CommonError},
        models::user::CreateUser,
    },
};
use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateUserDTO {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub user_name: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub nick_name: String,
    #[validate(length(min = 6, message = "Can not be empty"))]
    pub password: String,
}

impl Into<CreateUser> for CreateUserDTO {
    fn into(self) -> CreateUser {
        CreateUser {
            user_name: self.user_name,
            nick_name: self.nick_name,
            password: self.password,
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct UserLoginDTO {
    #[validate(length(min = 6, message = "Can not be empty"))]
    pub user_name: String,
    #[validate(length(min = 6, message = "Can not be empty"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

impl AuthBody {
    pub fn new(token: String) -> Self {
        AuthBody {
            access_token: token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub company: String,
    pub exp: i64,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ApiError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let secret = api::get_jwt_secret();
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|err| -> ApiError {
                CommonError {
                    message: format!("invalid token: {err}"),
                    code: 403,
                }
                .into()
            })?;

        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|err| -> ApiError {
            CommonError {
                message: format!("invalid token: {err}"),
                code: 403,
            }
            .into()
        })?;
        Ok(token_data.claims)
    }
}
