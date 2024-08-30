use async_trait::async_trait;

use crate::domain::{error::CommonError, models::{todo::CreateTodo, user::User}};

#[async_trait]
pub trait UserService:Send+Sync{
    async fn create(&self,user:CreateTodo)->Result<User,CommonError>;
    async fn get(&self,user_id:i32)->Result<User,CommonError>;
}