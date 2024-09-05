use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    api::dto::todo::{CreateTodoDTO, TodoDTO},
    domain::{
        error::ApiError,
        repositories::{repository::ResultPaging, todo::TodoQueryParams},
        services::todo::TodoService,
    },
};

pub async fn create(
    State(srv): State<Arc<dyn TodoService + 'static>>,
    Json(payload): Json<CreateTodoDTO>,
) -> Result<&'static str, ApiError> {
    let cloned = srv.clone();
    let _ = cloned
        .create(payload.into())
        .await
        .map_err(|e| -> ApiError { e.into() })?;

    Ok("success")
}

pub async fn list(
    State(srv): State<Arc<dyn TodoService + 'static>>,
    Json(payload): Json<TodoQueryParams>,
) -> Result<Json<ResultPaging<TodoDTO>>, ApiError> {
    let cloned = srv.clone();
    let todos = cloned
        .list(payload)
        .await
        .map_err(|e| -> ApiError { e.into() })?;
    Ok(Json(todos.into()))
}
