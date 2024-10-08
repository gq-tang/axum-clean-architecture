use crate::domain::models::todo::{CreateTodo, Todo};
use crate::domain::repositories::repository::{RepositoryResult, ResultPaging};
use crate::domain::repositories::todo::{TodoQueryParams, TodoRepository};
use crate::infrastructure::database::sqlite::DBConn;
use crate::infrastructure::models::todo::{CreateTodoPO, TodoPo};
use async_trait::async_trait;
use sqlx::QueryBuilder;
use std::sync::Arc;

pub struct TodoSqlxRepository {
    pool: Arc<DBConn>,
}

impl TodoSqlxRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        TodoSqlxRepository { pool: db }
    }
}

#[async_trait]
impl TodoRepository for TodoSqlxRepository {
    async fn create(&self, new_todo: &CreateTodo) -> RepositoryResult<i64> {
        let mut connect = self.pool.clone().acquire().await.unwrap();
        let t = CreateTodoPO::from(new_todo.clone());
        let res = sqlx::query("insert into todos(user_id,title,description) values (?,?,?)")
            .bind(t.user_id)
            .bind(t.title)
            .bind(t.description)
            .execute(&mut *connect)
            .await?
            .last_insert_rowid();
        Ok(res)
    }

    async fn list(&self, params: TodoQueryParams) -> RepositoryResult<ResultPaging<Todo>> {
        let mut connect = self.pool.clone().acquire().await.unwrap();
        let mut builder = QueryBuilder::new(
            r#"
        select * from todos where user_id =
        "#,
        );
        let mut count_builder = QueryBuilder::new(
            r#"
        select count(*) as total from todos where user_id=
        "#,
        );
        builder.push_bind(params.user_id);
        count_builder.push_bind(params.user_id);
        if let Some(title) = params.title {
            builder
                .push(" and title like ")
                .push_bind(format!("%{}%", title));
            count_builder
                .push(" and title like ")
                .push_bind(format!("%{}%", title));
        }

        let query = builder.build_query_as::<TodoPo>();
        let list = query.fetch_all(&mut *connect).await?;
        let count_query = count_builder.build_query_scalar::<i64>();
        let count = count_query.fetch_one(&mut *connect).await?;
        Ok(ResultPaging {
            total: count,
            items: list.into_iter().map(|v| v.into()).collect(),
        })
    }

    async fn get(&self, user_id: i64, todo_id: i64) -> RepositoryResult<Todo> {
        let mut connect = self.pool.clone().acquire().await.unwrap();
        let query = "select * from todos where id=? and user_id=? ";
        let todo = sqlx::query_as::<_, TodoPo>(&query)
            .bind(todo_id)
            .bind(user_id)
            .fetch_one(&mut *connect)
            .await?;

        Ok(todo.into())
    }

    async fn delete(&self, user_id: i64, todo_id: i64) -> RepositoryResult<()> {
        let mut connect = self.pool.clone().acquire().await.unwrap();

        let _ = sqlx::query("delete from todos where id=? and user_id=?")
            .bind(todo_id)
            .bind(user_id)
            .execute(&mut *connect)
            .await?;

        Ok(())
    }

    async fn completed(&self, user_id: i64, todo_id: i64, completed: bool) -> RepositoryResult<()> {
        let mut connect = self.pool.acquire().await.unwrap();

        let _ = sqlx::query("update todos set completed=? where id=? and user_id=?")
            .bind(completed)
            .bind(todo_id)
            .bind(user_id)
            .execute(&mut *connect)
            .await?;

        Ok(())
    }
}
