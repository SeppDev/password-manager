use sqlx::Result;

#[cfg(feature = "mysql")]
pub mod db {
    use sqlx::{Pool, mysql::MySqlQueryResult};

    pub const DB_SOLUTION: &str = "mysql";
    pub type DBSolution = sqlx::MySql;
    pub type DBPool = Pool<DBSolution>;
    pub type QueryResult = sqlx::Result<MySqlQueryResult>;
}

#[cfg(not(feature = "mysql"))]
pub mod db {
    use sqlx::{Pool, sqlite::SqliteQueryResult};

    pub const DB_SOLUTION: &str = "sqlite";
    pub type DBSolution = sqlx::Sqlite;
    pub type DBPool = Pool<DBSolution>;
    pub type QueryResult = sqlx::Result<SqliteQueryResult>;
}

use db::*;

mod accounts;
mod init;

pub struct Database {
    pub pool: DBPool,
}
impl Database {
    pub fn from(pool: DBPool) -> Self {
        Self { pool }
    }

    // #[cfg(feature = "mysql")]
    // pub async fn open(username: impl ToString, password: impl ToString) -> Result<Self> {
    //     let pool = DBPool::connect("mysql://user:pass@host/database")
    //         .await
    //         .unwrap();
    //     Ok(Self::from(pool))
    // }

    pub async fn new_memory() -> Result<Self> {
        let pool = DBPool::connect(format!("{DB_SOLUTION}::memory:").as_str())
            .await
            .unwrap();
        Ok(Self::from(pool))
    }
}
