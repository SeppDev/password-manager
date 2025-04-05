use sqlx::{Executor, Result, postgres::PgQueryResult};

pub mod db {
    use sqlx::Pool;

    pub type DBSolution = sqlx::Postgres;
    pub type DBPool = Pool<DBSolution>;
    pub type QueryResult<T> = sqlx::Result<T, sqlx::Error>;
}


pub mod db_config {
    pub const USERS_TABLE: &str = "users";
    pub const SESSIONS_TABLE: &str = "sessions";
    pub const DATA_TABLE: &str = "data";
}

use db::*;

pub mod accounts;
pub mod init;

pub struct Database {
    pub pool: DBPool,
}
impl Database {
    pub fn from(pool: DBPool) -> Self {
        Self { pool }
    }
    pub async fn open(url: &String) -> Result<Self> {
        let pool = DBPool::connect(&url).await.unwrap();
        Ok(Self::from(pool))
    }
    pub async fn execute(&self, query: impl ToString) -> db::QueryResult<PgQueryResult> {
        self.pool.execute(query.to_string().as_str()).await
    }
}
