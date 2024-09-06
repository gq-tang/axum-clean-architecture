use crate::domain::models::todo::CreateTodo;
use crate::domain::models::todo::Todo;
use crate::domain::repositories::repository::ResultPaging;
use serde::Deserialize;
use serde::Serialize;
use time::PrimitiveDateTime;

#[derive(Deserialize, Serialize)]
pub struct CreateTodoDTO {
    #[serde(skip)]
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

#[derive(Serialize, Debug)]
pub struct TodoDTO {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

impl Into<TodoDTO> for Todo {
    fn into(self) -> TodoDTO {
        TodoDTO {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            description: self.description,
            completed: self.completed,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl Into<ResultPaging<TodoDTO>> for ResultPaging<Todo> {
    fn into(self) -> ResultPaging<TodoDTO> {
        ResultPaging {
            total: self.total,
            items: self.items.into_iter().map(|todo| todo.into()).collect(),
        }
    }
}
