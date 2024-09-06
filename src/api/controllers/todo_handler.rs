use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    api::dto::todo::{CreateTodoDTO, TodoDTO},
    container::Container,
    domain::{
        error::ApiError,
        repositories::{repository::ResultPaging, todo::TodoQueryParams},
    },
};

pub async fn create_todo_handler(
    State(container): State<Arc<Container>>,
    Json(payload): Json<CreateTodoDTO>,
) -> Result<&'static str, ApiError> {
    let cloned = container.todo_service.clone();
    let _ = cloned
        .create(payload.into())
        .await
        .map_err(|e| -> ApiError { e.into() })?;

    Ok("success")
}

pub async fn list_todo_handler(
    State(container): State<Arc<Container>>,
    Json(payload): Json<TodoQueryParams>,
) -> Result<Json<ResultPaging<TodoDTO>>, ApiError> {
    let cloned = container.todo_service.clone();
    let todos = cloned
        .list(payload)
        .await
        .map_err(|e| -> ApiError { e.into() })?;
    Ok(Json(todos.into()))
}
