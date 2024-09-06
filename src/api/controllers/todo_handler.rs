use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    api::dto::{
        todo::{CreateTodoDTO, TodoDTO},
        user::Claims,
    },
    container::Container,
    domain::{
        error::ApiError,
        repositories::{repository::ResultPaging, todo::TodoQueryParams},
    },
};

pub async fn create_todo_handler(
    claims: Claims,
    State(container): State<Arc<Container>>,
    Json(mut payload): Json<CreateTodoDTO>,
) -> Result<&'static str, ApiError> {
    let cloned = container.todo_service.clone();
    payload.user_id = claims.sub;
    let _ = cloned
        .create(payload.into())
        .await
        .map_err(|e| -> ApiError { e.into() })?;

    Ok("success")
}

pub async fn list_todo_handler(
    claims: Claims,
    State(container): State<Arc<Container>>,
    Json(mut payload): Json<TodoQueryParams>,
) -> Result<Json<ResultPaging<TodoDTO>>, ApiError> {
    let cloned = container.todo_service.clone();
    payload.user_id = claims.sub;
    let todos = cloned
        .list(payload)
        .await
        .map_err(|e| -> ApiError { e.into() })?;
    Ok(Json(todos.into()))
}
