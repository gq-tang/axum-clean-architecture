use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub user_name: String,
    pub nick_name: String,
    pub password: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(Clone)]
pub struct CreateUser {
    pub user_name: String,
    pub nick_name: String,
    pub password: String,
}
