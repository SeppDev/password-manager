#[cfg(test)]
mod tests {
    use bcrypt::hash;
    use sqlx::PgPool;
    use sqlx::pool::PoolOptions;
    use sqlx::{Connection, Pool, Postgres, postgres::PgRow, sqlx_macros};
    use std::env;

    use crate::database::Database;

    async fn create_database() -> anyhow::Result<Database> {
        dotenv::dotenv().ok();

        let pool = PoolOptions::new()
            .min_connections(0)
            .max_connections(5)
            .test_before_acquire(true)
            .connect(&env::var("DATABASE_URL")?)
            .await?;

        let db = Database::from(pool);
        db.init_connection().await;
        

        Ok(db)
    }


    #[sqlx_macros::test]
    async fn it_connects() -> anyhow::Result<()> {
        create_database().await?;
        Ok(())
    }

    #[sqlx_macros::test]

    async fn account_creation() -> anyhow::Result<()> {
        let db = create_database().await?;

        db.create_account("John", "password").await?;

        Ok(())
    }
}
