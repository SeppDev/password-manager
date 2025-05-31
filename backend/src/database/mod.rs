use accounts::UserData;
use sqlx::postgres::PgRow;
use std::sync::Arc;
use tokio::sync::Mutex;

use serde::{Deserialize, Serialize};
use sqlx::pool::PoolOptions;
use sqlx::query::{Query, QueryAs};
use sqlx::{Executor, Result, Transaction};
use sqlx::{FromRow, Pool};

pub mod db {
    use sqlx::postgres::PgQueryResult;

    pub type Arguments = sqlx::postgres::PgArguments;
    pub type DBSolution = sqlx::Postgres;
    pub type QueryResult<T = PgQueryResult> = sqlx::Result<T, sqlx::Error>;
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

pub enum DBPool {
    Normal(Pool<DBSolution>),
    Testing(Arc<Mutex<Transaction<'static, DBSolution>>>),
}

pub struct Database {
    pub pool: DBPool,
    pub jwt: JWTSession<UserClaims>,
}

impl Database {
    #[cfg(test)]
    pub async fn test(url: &str) -> Result<Self> {
        let pool = PoolOptions::new()
            .test_before_acquire(true)
            .connect(&url)
            .await?;

        // let transaction = Box::new(pool.begin().await?);
        let transaction = Arc::new(Mutex::new(pool.begin().await?));

        Ok(Self {
            pool: DBPool::Testing(transaction),
            jwt: JWTSession::new(),
        })
    }

    pub async fn open(url: &str) -> Result<Self> {
        let pool = PoolOptions::new()
            .test_before_acquire(true)
            .connect(&url)
            .await?;

        Ok(Self {
            pool: DBPool::Normal(pool),
            jwt: JWTSession::new(),
        })
    }

    #[cfg(not(test))]
    pub async fn executor(&self) -> &Pool<DBSolution> {
        match &self.pool {
            DBPool::Normal(p) => p,
            _ => panic!(),
        }
    }

    #[cfg(test)]
    pub async fn executor(&self) -> tokio::sync::MutexGuard<'_, Transaction<'static, DBSolution>> {
        match &self.pool {
            DBPool::Testing(tx) => tx.lock().await,
            _ => panic!("Tried to use normal executor in test mode"),
        }
    }

    pub async fn execute(&self, query: Query<'_, DBSolution, Arguments>) -> QueryResult {
        #[allow(unused_mut)]
        let mut executor = self.executor().await;
        executor.execute(query).await
    }

    pub async fn fetch_one<'q, T>(
        &self,
        query: QueryAs<'q, DBSolution, UserData, Arguments>,
    ) -> QueryResult<T>
    where
        T: for<'r> FromRow<'r, PgRow>,
    {
        #[allow(unused_mut)]
        let mut executor = self.executor().await;
        let row = executor.fetch_one(query).await?;

        T::from_row(&row)
    }
    
    pub async fn query(&self, query: &str) -> QueryResult {
        #[allow(unused_mut)]
        let mut executor = self.executor().await;
        let query = sqlx::query(query);
        executor.execute(query).await
    }
}
