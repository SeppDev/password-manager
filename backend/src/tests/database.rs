#[cfg(test)]
mod tests {
    use sqlx::pool::PoolOptions;
    use sqlx::sqlx_macros;
    use std::env;

    use crate::database::Database;

    async fn create_database() -> anyhow::Result<Database> {
        dotenv::dotenv().ok();

        let database_url = env::var("TEST_DATABASE_URL").expect("Expected database_url");

        let pool = PoolOptions::new()
            .min_connections(0)
            .max_connections(5)
            .test_before_acquire(true)
            .connect(&database_url)
            .await?;

        let db = Database::from(pool);
        db.test_init_connection().await;

        Ok(db)
    }

    #[sqlx_macros::test]
    async fn it_connects() -> anyhow::Result<()> {
        create_database().await?;
        Ok(())
    }
}
