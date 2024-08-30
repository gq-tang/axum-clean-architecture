use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    api::dto::todo::CreateTodoDTO,
    domain::{error::ApiError, services::todo::TodoService},
};

pub async fn create(
    State(srv): State<Arc<dyn TodoService + 'static>>,
    Json(payload): Json<CreateTodoDTO>,
) -> Result<&'static str, ApiError> {
    let cloned = srv.clone();
    if let Err(e) = cloned.create(payload.into()).await {
        return Err(e.into());
    }
    Ok("success")
}
