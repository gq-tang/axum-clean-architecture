use crate::domain::{
    error::CommonError,
    models::user::{CreateUser, User},
    repositories::user::UserRepository,
    services::user::UserService,
};
use async_trait::async_trait;
use dotenv::dotenv;
use std::sync::Arc;

pub struct UserServiceImpl {
    repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        dotenv().ok();

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

    async fn login(&self, user_name: String, password: String) -> Result<User, CommonError> {
        let password = md5::compute(password);
        let password = format!("{:X}", password);

        let user = self
            .repository
            .get_by_username(user_name)
            .await
            .map_err(|e| -> CommonError { e.into() })?;
        if user.password != password {
            return Err(CommonError {
                message: "Account or password error".to_string(),
                code: 404,
            });
        }

        Ok(user)
    }
}
