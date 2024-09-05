use time::PrimitiveDateTime;

use crate::domain::models::user::{CreateUser, User};

pub struct CreateUserPO {
    pub user_name: String,
    pub nick_name: String,
    pub password: String,
}

impl From<CreateUser> for CreateUserPO {
    fn from(value: CreateUser) -> Self {
        let password = md5::compute(value.password);
        let password = format!("{:X}", password);
        CreateUserPO {
            user_name: value.user_name,
            nick_name: value.nick_name,
            password: password,
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct UserPO {
    pub id: i32,
    pub user_name: String,
    pub nick_name: String,
    pub password: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

impl Into<User> for UserPO {
    fn into(self) -> User {
        User {
            id: self.id,
            user_name: self.user_name,
            nick_name: self.nick_name,
            password: self.password,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
