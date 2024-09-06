use serde::Deserialize;

use crate::domain::models::user::CreateUser;

#[derive(Deserialize)]
pub struct CreateUserDTO {
    pub user_name: String,
    pub nick_name: String,
    pub password: String,
}

impl Into<CreateUser> for CreateUserDTO {
    fn into(self) -> CreateUser {
        CreateUser {
            user_name: self.user_name,
            nick_name: self.nick_name,
            password: self.password,
        }
    }
}
