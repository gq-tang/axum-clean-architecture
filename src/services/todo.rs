use crate::domain::{
    error::CommonError,
    models::todo::{CreateTodo, Todo},
    repositories::{
        repository::ResultPaging,
        todo::{TodoQueryParams, TodoRepository},
    },
    services::todo::TodoService,
};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct TodoServiceImpl {
    repository: Arc<dyn TodoRepository>,
}

impl TodoServiceImpl {
    pub fn new(repository: Arc<dyn TodoRepository>) -> Self {
        TodoServiceImpl { repository }
    }
}

#[async_trait]
impl TodoService for TodoServiceImpl {
    async fn create(&self, todo: CreateTodo) -> Result<i64, CommonError> {
        let cloned = todo.clone();
        self.repository
            .create(&cloned)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn list(&self, param: TodoQueryParams) -> Result<ResultPaging<Todo>, CommonError> {
        self.repository
            .list(param)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get(&self, user_id: i64, todo_id: i64) -> Result<Todo, CommonError> {
        self.repository
            .get(user_id, todo_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn delete(&self, user_id: i64, todo_id: i64) -> Result<(), CommonError> {
        self.repository
            .delete(user_id, todo_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }
}
