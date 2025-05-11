use serde::{Deserialize, Serialize};
use sqlx::{Executor, Result, postgres::PgQueryResult};

pub mod db {
    use sqlx::Pool;

    pub type DBSolution = sqlx::Postgres;
    pub type DBPool = Pool<DBSolution>;
    pub type QueryResult<T> = sqlx::Result<T, sqlx::Error>;
}

pub mod config {
    pub const USERS_TABLE: &str = "users";
    pub const VAULT_TABLE: &str = "vaults";
}

use db::*;

use crate::jwt::JWTSession;

pub mod accounts;
pub mod init;

#[derive(Serialize, Deserialize)]
pub struct UserClaims {
    pub user_id: i64,
}

pub struct Database {
    pub pool: DBPool,
    pub jwt: JWTSession<UserClaims>,
}
impl Database {
    pub fn from(pool: DBPool) -> Self {
        Self {
            pool,
            jwt: JWTSession::new(),
        }
    }
    pub async fn open(url: &String) -> Result<Self> {
        let pool = DBPool::connect(&url).await?;
        Ok(Self::from(pool))
    }
    pub async fn execute(&self, query: impl ToString) -> db::QueryResult<PgQueryResult> {
        self.pool.execute(query.to_string().as_str()).await
    }
}
