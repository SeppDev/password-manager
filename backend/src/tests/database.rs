#[cfg(test)]
mod tests {
    use crate::database::Database;
    use tokio_rusqlite::Result;
    use tokio::runtime::Runtime;

    const PASSWORD: &str = "password123";

    async fn create_database() -> Result<Database> {
        let db = Database::new_memory().await?;
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
        rt.block_on(db.create_account("John".into(), PASSWORD.into())).unwrap();
    }

    #[test]
    fn account_creation_same_username() {
        let rt = Runtime::new().unwrap();
        let db = rt.block_on(create_database()).unwrap();
        rt.block_on(db.create_account("John".into(), PASSWORD.into())).unwrap();
        rt.block_on(db.create_account("John".into(), PASSWORD.into())).unwrap_err();
    }
}
