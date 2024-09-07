use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    api::dto::{
        response::ApiResponse,
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
) -> Result<ApiResponse<()>, ApiError> {
    let cloned = container.todo_service.clone();
    payload.user_id = claims.sub;
    let _ = cloned
        .create(payload.into())
        .await
        .map_err(|e| -> ApiError { e.into() })?;

    Ok(ApiResponse::success())
}

pub async fn list_todo_handler(
    claims: Claims,
    State(container): State<Arc<Container>>,
    Query(mut payload): Query<TodoQueryParams>,
) -> Result<ApiResponse<ResultPaging<TodoDTO>>, ApiError> {
    let cloned = container.todo_service.clone();
    payload.user_id = claims.sub;
    println!("debug: {:?}", payload);
    let todos = cloned
        .list(payload)
        .await
        .map_err(|e| -> ApiError { e.into() })?;
    Ok(ApiResponse::new(todos.into()))
}

pub async fn get_todo_handler(
    cliams: Claims,
    State(container): State<Arc<Container>>,
    Path(todo_id): Path<i64>,
) -> Result<ApiResponse<TodoDTO>, ApiError> {
    let cloned = container.todo_service.clone();
    let user_id = cliams.sub;
    let todo = cloned
        .get(user_id, todo_id)
        .await
        .map_err(|e| -> ApiError { e.into() })?;

    Ok(ApiResponse::new(todo.into()))
}

pub async fn delte_todo_handler(
    claims: Claims,
    State(container): State<Arc<Container>>,
    Path(todo_id): Path<i64>,
) -> Result<ApiResponse<()>, ApiError> {
    let cloned = container.todo_service.clone();
    let user_id = claims.sub;
    cloned
        .delete(user_id, todo_id)
        .await
        .map_err(|e| -> ApiError { e.into() })?;
    Ok(ApiResponse::success())
}
