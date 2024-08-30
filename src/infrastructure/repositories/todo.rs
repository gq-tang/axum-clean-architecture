use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::repositories::todo::{TodoQueryParams, TodoRepository};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging}; 
use crate::domain::models::todo::{CreateTodo, Todo};
use crate::infrastructure::database::sqlite::DBConn;
use crate::infrastructure::models::todo::{CreateTodoPO, TodoPo};
use sqlx::Row;


pub struct TodoSqlxRepository{
    pool:Arc<DBConn>
}

impl TodoSqlxRepository{
    pub fn new(db:Arc<DBConn>)->Self{
        TodoSqlxRepository{pool:db}
    }
}

#[async_trait]
impl TodoRepository for TodoSqlxRepository {
    async fn create(&self, new_todo: &CreateTodo) -> RepositoryResult<i64> {
        let mut connect=self.pool.clone().acquire().await.unwrap();
        let t=CreateTodoPO::from(new_todo.clone());
        let res=sqlx::query("insert into todos(user_id,title,description) values (?1,?2,?3)")
        .bind(t.user_id)
        .bind(t.title)
        .bind(t.description)
        .execute(&mut *connect)
        .await?
        .last_insert_rowid(); 
        Ok(res) 
    }

    async fn list(&self, params: TodoQueryParams) -> RepositoryResult<ResultPaging<Todo>>{
        let mut connect=self.pool.clone().acquire().await.unwrap();
        let mut query="select * from todos".to_string();
        let mut query_count="select count(1) as total from todos".to_string();

        let mut conditions=Vec::new();
        conditions.push(format!("user_id={}",params.user_id));

        if let Some(title)=params.title.clone(){
            conditions.push(format!("title like %{}%",title)); 
        }
        if conditions.len()>0{
            query=query+" where "+&conditions.join(" and "); 
            query_count=query_count+ " where "+&conditions.join(" and "); 
        }
        
        query = format!("{} limit {} offset {}",query,params.limit(),params.offset());

        let count=sqlx::query(&query_count)
        .fetch_one(&mut *connect)
        .await?;
        let count=count.get::<i64,_>(0);

        let list=sqlx::query_as::<_,TodoPo>(&query)
        .fetch_all(&mut *connect)
        .await?; 
        Ok(ResultPaging { total: count, items: list.into_iter().map(|v|v.into()).collect() })
    }

    async fn get(&self,user_id:i64, todo_id: i64) -> RepositoryResult<Todo>{
        let mut connect=self.pool.clone().acquire().await.unwrap();
        let query="select * from todos where id=? ";
        let todo=sqlx::query_as::<_,TodoPo>(&query)
        .bind(todo_id)
        .fetch_one(&mut *connect)
        .await?;

        Ok(todo.into())
    }

    async fn delete(&self,user_id:i64, todo_id: i64) -> RepositoryResult<()>{
        let mut connect=self.pool.clone().acquire().await.unwrap();

        let res=sqlx::query("delete from todos where id=? and user_id=?")
        .bind(todo_id)
        .bind(user_id)
        .execute(&mut *connect)
        .await?;

        Ok(())
    }
}