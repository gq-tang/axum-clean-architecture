use crate::domain::models::todo::{CreateTodo, Todo};
use time::PrimitiveDateTime;


pub struct CreateTodoPO{
    pub user_id:i64,
    pub title:String,
    pub description:String,
}

impl From<CreateTodo> for CreateTodoPO{
    fn from(t: CreateTodo) -> Self {
        CreateTodoPO{
            user_id:t.user_id,
            title:t.title,
            description:t.description,
        }  
    }
}

#[derive(sqlx::FromRow)]
pub struct TodoPo{
    pub id:i64,
    pub user_id:i64,
    pub title:String,
    pub description:String,
    pub completed:bool,
    pub created_at:PrimitiveDateTime,
    pub updated_at:PrimitiveDateTime,
}

impl Into<Todo> for TodoPo{
    fn into(self) -> Todo {
        Todo{
           id:self.id,
           user_id:self.user_id,
           title:self.title,
           description:self.description,
           completed:self.completed,
           created_at:self.created_at,
           updated_at:self.updated_at,
        }
    }
}