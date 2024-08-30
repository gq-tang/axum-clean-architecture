use serde::Deserialize;
use serde::Serialize;

use crate::domain::models::todo::CreateTodo;

#[derive(Deserialize, Serialize)]
pub struct CreateTodoDTO {
    pub user_id: i64,
    pub title: String,
    pub description: String,
}

impl Into<CreateTodo> for CreateTodoDTO {
    fn into(self) -> CreateTodo {
        CreateTodo {
            user_id: self.user_id,
            title: self.title,
            description: self.description,
        }
    }
}
