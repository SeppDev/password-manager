#[cfg(test)]
mod tests {
    use bcrypt::{hash, verify};
    use fake::faker::internet::en::Username;
    use fake::{Fake, faker};
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
        db.test_init_connection().await;

        Ok(db)
    }

    fn generate_fake_username() -> String {
        Username().fake()
    }

    #[sqlx_macros::test]
    async fn it_connects() -> anyhow::Result<()> {
        create_database().await?;
        Ok(())
    }

    #[sqlx_macros::test]
    async fn account_creation() -> anyhow::Result<()> {
        let db = create_database().await?;
        let username: String = Username().fake();

        db.create_account(&username, "password").await?;    
        Ok(())
    }

    #[sqlx_macros::test]
    async fn password_hashing() -> anyhow::Result<()> {
        let db = create_database().await?;

        let username: String = Username().fake();
        const PASSWORD: &'static str = "password";

        db.create_account(&username, PASSWORD).await?;

        let user = db.get_user_by_name(&username).await?;
        let correct = verify(PASSWORD, &user.password).unwrap();
        assert!(correct);

        Ok(())
    }

    #[sqlx_macros::test]
    async fn wrong_password_hash() -> anyhow::Result<()> {
        let db = create_database().await?;

        let username: String = Username().fake();
        const PASSWORD: &'static str = "password";
        const WRONG_PASSWORD: &'static str = "wrong_password";

        db.create_account(&username, PASSWORD).await?;

        let user = db.get_user_by_name(&username).await?;
        let correct = verify(WRONG_PASSWORD, &user.password).unwrap();
        assert!(!correct);

        Ok(())
    }
}
