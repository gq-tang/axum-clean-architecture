use crate::domain::{
    error::CommonError,
    models::user::{CreateUser, User},
    repositories::user::UserRepository,
    services::user::UserService,
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct UserServiceImpl {
    repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        UserServiceImpl { repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create(&self, user: CreateUser) -> Result<i64, CommonError> {
        self.repository
            .create(user)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get_by_username(&self, user_name: String) -> Result<User, CommonError> {
        self.repository
            .get_by_username(user_name)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }
}
