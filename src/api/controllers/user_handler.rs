use std::{sync::Arc, time::Duration};

use axum::{extract::State, Json};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use validator::Validate;

use crate::{
    api::{
        self,
        dto::{
            response::ApiResponse,
            user::{AuthBody, Claims, CreateUserDTO, UserLoginDTO},
        },
    },
    container::Container,
    domain::error::ApiError,
};

pub async fn register_handler(
    State(container): State<Arc<Container>>,
    Json(payload): Json<CreateUserDTO>,
) -> Result<ApiResponse<()>, ApiError> {
    if let Err(e) = payload.validate() {
        return Err(ApiError::new(404, e));
    }
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
    if let Err(e) = payload.validate() {
        return Err(ApiError::new(404, e));
    }

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
    .map_err(|_| -> ApiError { ApiError::new(403, "jwt encode failed") })?;
    let res = AuthBody::new(token);
    Ok(ApiResponse::new(res))
}
