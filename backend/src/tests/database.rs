#[cfg(test)]
mod tests {
    use crate::database::Database;
    use dotenv::dotenv;
    use sqlx::Result;
    use tokio::runtime::Runtime;

    const PASSWORD: &str = "password123";

    async fn create_database() -> Result<Database> {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");

        let db = Database::open(&database_url).await?;
        db.init_connection().await;
        Ok(db)
    }

    #[test]
    fn database_creation() {
        let rt = Runtime::new().unwrap();
        rt.block_on(create_database()).unwrap();
    }

    #[test]
    fn account_creation() {
        let rt = Runtime::new().unwrap();
        let db = rt.block_on(create_database()).unwrap();
        rt.block_on(db.create_account(&"John".into(), &PASSWORD.into()))
            .unwrap();
    }

    #[test]
    fn account_creation_same_username() {
        let rt = Runtime::new().unwrap();
        let db = rt.block_on(create_database()).unwrap();
        rt.block_on(db.create_account(&"John".into(), &PASSWORD.into()))
            .unwrap();
        rt.block_on(db.create_account(&"John".into(), &PASSWORD.into()))
            .unwrap_err();
    }

    #[test]
    fn account_id_check() {
        let rt = Runtime::new().unwrap();
        let db = rt.block_on(create_database()).unwrap();
        rt.block_on(db.create_account(&"John".into(), &PASSWORD.into()))
            .unwrap();
        rt.block_on(db.create_account(&"Jane".into(), &PASSWORD.into()))
            .unwrap();

        let john = rt.block_on(db.get_user_by_name(&"John".into())).unwrap();
        let jane = rt.block_on(db.get_user_by_name(&"Jane".into())).unwrap();
        assert!(john.id != jane.id)
    }
}
