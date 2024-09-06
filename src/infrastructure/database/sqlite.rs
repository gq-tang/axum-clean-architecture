use std::env;

use sqlx::{Pool, Sqlite, SqlitePool};

use crate::domain::constants::SQLITE_DB_URI;

pub type DBConn = Pool<Sqlite>;

pub async fn db_pool() -> DBConn {
    let database_url = env::var(SQLITE_DB_URI).expect(&*format!("{} must be set", SQLITE_DB_URI));

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("cann't connect sqlite");
    pool
}
