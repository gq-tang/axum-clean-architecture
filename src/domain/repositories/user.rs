use super::repository::RepositoryResult;
use crate::domain::models::user::{CreateUser, User};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: CreateUser) -> RepositoryResult<i64>;
    async fn get_by_username(&self, user_name: String) -> RepositoryResult<User>;
}
