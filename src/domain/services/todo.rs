use crate::domain::{
    error::CommonError,
    models::todo::{CreateTodo, Todo},
    repositories::{repository::ResultPaging, todo::TodoQueryParams},
};
use async_trait::async_trait;

#[async_trait]
pub trait TodoService: Send + Sync {
    async fn create(&self, todo: CreateTodo) -> Result<i64, CommonError>;
    async fn list(&self, param: TodoQueryParams) -> Result<ResultPaging<Todo>, CommonError>;
    async fn get(&self, user_id: i64, todo_id: i64) -> Result<Todo, CommonError>;
    async fn delete(&self, user_id: i64, todo_id: i64) -> Result<(), CommonError>;
    async fn completed(
        &self,
        user_id: i64,
        todo_id: i64,
        completed: bool,
    ) -> Result<(), CommonError>;
}
