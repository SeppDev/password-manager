use super::Database;
use super::db_config::{SESSIONS_TABLE, USERS_TABLE};

impl Database {
    #[cfg(test)]
    pub async fn test_init_connection(&self) {
        let query = format!(
            "
            DROP TABLE IF EXISTS {SESSIONS_TABLE};
            DROP TABLE IF EXISTS {USERS_TABLE};
        "
        );
        self.execute(query).await.unwrap();
        self.init_connection().await
    }

    pub async fn init_connection(&self) {
        self._init_connection(USERS_TABLE, SESSIONS_TABLE).await;
    }

    async fn _init_connection(&self, users: &str, sessions: &str) {
        let query = format!(
            "CREATE TABLE IF NOT EXISTS {users} (
                id         BIGSERIAL PRIMARY KEY,
                name       TEXT NOT NULL UNIQUE,
                password   TEXT NOT NULL,
                data       BYTEA NOT NULL 
            );",
        );

        self.execute(query).await.unwrap();

        let query = format!(
            "CREATE TABLE IF NOT EXISTS {sessions} (
            token      TEXT NOT NULL PRIMARY KEY,
            expires_at TIMESTAMPTZ NOT NULL,
            user_id    BIGINT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES {users}(id)
        )"
        );

        self.execute(query).await.unwrap();
    }
}
