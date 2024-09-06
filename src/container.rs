use std::sync::Arc;

use crate::{
    domain::{
        repositories::{todo::TodoRepository, user::UserRepository},
        services::{todo::TodoService, user::UserService},
    },
    infrastructure::{
        database::sqlite,
        repositories::{todo::TodoSqlxRepository, user::UserSqlxRepository},
    },
    services::{todo::TodoServiceImpl, user::UserServiceImpl},
};

pub struct Container {
    pub todo_service: Arc<dyn TodoService>,
    pub user_service: Arc<dyn UserService>,
}

impl Container {
    pub async fn new() -> Self {
        let pool = Arc::new(sqlite::db_pool().await);
        let todo_repository: Arc<dyn TodoRepository> =
            Arc::new(TodoSqlxRepository::new(pool.clone()));
        let todo_service = Arc::new(TodoServiceImpl::new(todo_repository));

        let user_repository: Arc<dyn UserRepository> =
            Arc::new(UserSqlxRepository::new(pool.clone()));
        let user_service = Arc::new(UserServiceImpl::new(user_repository));

        Container {
            todo_service: todo_service,
            user_service: user_service,
        }
    }
}
