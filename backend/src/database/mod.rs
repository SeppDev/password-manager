use sqlx::Result;

pub mod db {
    use sqlx::{Pool, postgres::PgQueryResult};

    pub const DB_SOLUTION: &str = "mysql";
    pub type DBSolution = sqlx::Postgres;
    pub type DBPool = Pool<DBSolution>;
    pub type QueryResult = sqlx::Result<PgQueryResult>;
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
}
