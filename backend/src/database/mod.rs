use sqlx::{Executor, Result};

pub mod db {
    use sqlx::{Pool, postgres::PgQueryResult};

    pub type DBSolution = sqlx::Postgres;
    pub type DBPool = Pool<DBSolution>;
    pub type QueryResult = sqlx::Result<PgQueryResult>;
}

#[cfg(test)]
pub mod db_config {
    pub const USERS_TABLE: &str = "temp_users";
}
#[cfg(not(test))]
pub mod db_config {
    pub const USERS_TABLE: &str = "users";
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
    pub async fn execute(&self, query: impl ToString) -> db::QueryResult {
        self.pool.execute(query.to_string().as_str()).await
    }
}
