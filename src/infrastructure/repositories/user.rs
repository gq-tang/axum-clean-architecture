use crate::{
    domain::{
        models::user::{CreateUser, User},
        repositories::{repository::RepositoryResult, user::UserRepository},
    },
    infrastructure::{
        database::sqlite::DBConn,
        models::user::{CreateUserPO, UserPO},
    },
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct UserSqlxRepository {
    pool: Arc<DBConn>,
}

impl UserSqlxRepository {
    pub fn new(pool: Arc<DBConn>) -> Self {
        UserSqlxRepository { pool }
    }
}

#[async_trait]
impl UserRepository for UserSqlxRepository {
    async fn create(&self, user: CreateUser) -> RepositoryResult<i64> {
        let mut connect = self.pool.acquire().await.unwrap();
        let user: CreateUserPO = user.into();
        let res = sqlx::query("insert into users(user_name,nick_name,password)values(?,?,?)")
            .bind(user.user_name)
            .bind(user.nick_name)
            .bind(user.password)
            .execute(&mut *connect)
            .await?
            .last_insert_rowid();

        Ok(res)
    }
    async fn get_by_username(&self, user_name: String) -> RepositoryResult<User> {
        let mut connect = self.pool.acquire().await.unwrap();
        let user = sqlx::query_as::<_, UserPO>("select * from users where user_name=?")
            .bind(user_name)
            .fetch_one(&mut *connect)
            .await?;
        Ok(user.into())
    }
}
