use super::Database;
use super::db_config::USERS_TABLE;

#[cfg(test)]
fn temp_prefix(string: &str) -> String {
    format!("temp_{string}")
}

impl Database {
    #[cfg(test)]
    pub async fn test_init_connection(&self) {
        let query = format!(
            "
            DROP TABLE IF EXISTS {USERS_TABLE};
        "
        );
        self.execute(query).await.unwrap();
        self._init_connection(temp_prefix(USERS_TABLE).as_str())
            .await
    }

    pub async fn init_connection(&self) {
        self._init_connection(USERS_TABLE).await;
    }

    async fn _init_connection(&self, users: &str) {
        let query = format!(
            "CREATE TABLE IF NOT EXISTS {users} (
                id         BIGSERIAL PRIMARY KEY,
                name       TEXT NOT NULL UNIQUE,
                password   TEXT NOT NULL,
                data       BYTEA NOT NULL
            );",
        );

        self.execute(query).await.unwrap();
    }
}
