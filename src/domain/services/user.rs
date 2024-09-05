use async_trait::async_trait;

use crate::domain::{
    error::CommonError,
    models::{ 
        user::{CreateUser, User},
    },
};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn create(&self, user: CreateUser) -> Result<i64, CommonError>;
    async fn get_by_username(&self, user_name: String) -> Result<User, CommonError>;
}
