use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;

use crate::domain::{
    constants,
    error::{ApiError, CommonError},
    models::user::CreateUser,
};

#[derive(Deserialize)]
pub struct CreateUserDTO {
    pub user_name: String,
    pub nick_name: String,
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

#[derive(Deserialize)]
pub struct UserLoginDTO {
    pub user_name: String,
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
        let secret = env::var(constants::JWT_SECRET).unwrap_or_else(|_| "verysecret".to_string());
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| -> ApiError {
                CommonError {
                    message: "invalid token".to_string(),
                    code: 403,
                }
                .into()
            })?;

        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| -> ApiError {
            CommonError {
                message: "invalid token".to_string(),
                code: 403,
            }
            .into()
        })?;
        Ok(token_data.claims)
    }
}
