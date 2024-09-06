use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{api::dto::user::CreateUserDTO, container::Container, domain::error::ApiError};

pub async fn register_handler(
    State(container): State<Arc<Container>>,
    Json(payload): Json<CreateUserDTO>,
) -> Result<&'static str, ApiError> {
    let cloned = container.user_service.clone();
    let _ = cloned
        .create(payload.into())
        .await
        .map_err(|e| -> ApiError { e.into() })?;
    Ok("success")
}
