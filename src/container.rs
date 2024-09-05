use std::sync::Arc;

use crate::{
    domain::{repositories::todo::TodoRepository, services::todo::TodoService},
    infrastructure::{
        database::{ sqlite},
        repositories::todo::TodoSqlxRepository,
    },
    services::todo::TodoServiceImpl,
};

pub struct Container {
    pub todo_service: Arc<dyn TodoService>,
}

impl Container {
    pub async fn new() -> Self {
        let pool = Arc::new(sqlite::db_pool().await);
        let todo_repository: Arc<dyn TodoRepository> = Arc::new(TodoSqlxRepository::new(pool));
        let todo_server = Arc::new(TodoServiceImpl::new(todo_repository));
        Container {
            todo_service: todo_server,
        }
    }
}
