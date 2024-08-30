use serde::Deserialize;
use time::PrimitiveDateTime;

#[derive(Clone,Deserialize)]
pub struct Todo{
    pub id:i64,
    pub user_id:i64,
    pub title:String,
    pub description:String,
    pub completed:bool,
    pub created_at:PrimitiveDateTime,
    pub updated_at:PrimitiveDateTime,
}

#[derive(Clone)]
pub struct CreateTodo{
    pub user_id:i64,
    pub title:String,
    pub description:String,
}