#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use sqlx::{pool::PoolOptions, sqlx_macros};
    use tokio::sync::Mutex;

    use crate::{
        database::{DBPool, Database},
        jwt::JWTSession,
    };

    impl Database {
        pub async fn test() -> Database {
            dotenv::dotenv().ok();

            let database_url = std::env::var("DATABASE_URL").expect("Expected database_url");
            let pool = PoolOptions::new()
                .test_before_acquire(true)
                .connect(&database_url)
                .await
                .expect("Failed connectin to database");

            let transaction = pool.begin().await.expect("Failed to open transaction");
            let transaction = Arc::new(Mutex::new(transaction));

            let db = Self {
                pool: DBPool::Testing(transaction),
                jwt: JWTSession::new(),
            };

            db.init_connection()
                .await
                .expect("Failed to initialize database");

            db
        }
    }

    #[sqlx_macros::test]
    async fn it_connects() {
        Database::test().await;
    }
}
