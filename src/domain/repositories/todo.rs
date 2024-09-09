use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::models::todo::{CreateTodo, Todo};

use super::repository::{
    QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoQueryParams {
    #[serde(skip)]
    pub user_id: i64,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub title: Option<String>,
}

impl QueryParams for TodoQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn create(&self, new_todo: &CreateTodo) -> RepositoryResult<i64>;
    async fn list(&self, params: TodoQueryParams) -> RepositoryResult<ResultPaging<Todo>>;
    async fn get(&self, user_id: i64, todo_id: i64) -> RepositoryResult<Todo>;
    async fn delete(&self, user_id: i64, todo_id: i64) -> RepositoryResult<()>;
    async fn completed(&self,user_id:i64,todo_id:i64,completed:bool)->RepositoryResult<()>; 
}
