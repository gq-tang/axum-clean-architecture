use std::{sync::Arc, time::Duration};

use axum::{extract::State, Json};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{
    api::{
        self,
        dto::{
            response::ApiResponse,
            user::{AuthBody, Claims, CreateUserDTO, UserLoginDTO},
        },
    },
    container::Container,
    domain::error::{ApiError, CommonError},
};

pub async fn register_handler(
    State(container): State<Arc<Container>>,
    Json(payload): Json<CreateUserDTO>,
) -> Result<ApiResponse<()>, ApiError> {
    let cloned = container.user_service.clone();
    let _ = cloned
        .create(payload.into())
        .await
        .map_err(|e| -> ApiError { e.into() })?;
    Ok(ApiResponse::success())
}

pub async fn login(
    State(container): State<Arc<Container>>,
    Json(payload): Json<UserLoginDTO>,
) -> Result<ApiResponse<AuthBody>, ApiError> {
    let cloned = container.user_service.clone();
    let user = cloned
        .login(payload.user_name, payload.password)
        .await
        .map_err(|e| -> ApiError { e.into() })?;

    let exp = Utc::now() + Duration::from_secs(3600 * 3);
    let claims = Claims {
        sub: user.id,
        company: "TMD".to_string(),
        exp: exp.timestamp(),
    };
    let secret = api::get_jwt_secret();
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| -> ApiError {
        CommonError {
            message: "jwt encode failed".to_string(),
            code: 403,
        }
        .into()
    })?;
    let res = AuthBody::new(token);
    Ok(ApiResponse::new(res))
}
