use async_trait::async_trait;

use crate::domain::{
    error::CommonError,
    models::user::{CreateUser, User},
};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn create(&self, user: CreateUser) -> Result<i64, CommonError>;
    async fn login(&self, user_name: String, password: String) -> Result<User, CommonError>;
}
