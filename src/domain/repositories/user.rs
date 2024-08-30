use super::repository::RepositoryResult;
use crate::domain::models::user::{CreateUser, User};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: CreateUser) -> RepositoryResult<User>;
    async fn get(&self,user_id:i64)->RepositoryResult<User>; 
}
