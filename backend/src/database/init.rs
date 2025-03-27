use super::Database;
use super::db_config::USERS_TABLE;

impl Database {
    #[cfg(test)]
    pub async fn init_connection(&self) {
        let query = format!(
            "
            DROP TABLE IF EXISTS {USERS_TABLE};

            CREATE TEMPORARY TABLE {USERS_TABLE} (
                id       BIGSERIAL PRIMARY KEY,
                name     TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                data     BYTEA NOT NULL
            )",
        );
        self.execute(query).await.unwrap();

        let query = "CREATE TABLE IF NOT EXISTS sessions (
                token      TEXT NOT NULL PRIMARY KEY,
                expires_at TIMESTAMPTZ NOT NULL,
                user_id    INTEGER NOT NULL,
                FOREIGN KEY (user_id) REFERENCES users(id)
            )";

        self.execute(query).await.unwrap();
    }

    #[cfg(not(test))]
    pub async fn init_connection(&self) {
        let query = format!(
            "CREATE TABLE IF NOT EXISTS {USERS_TABLE} (
                id       BIGSERIAL PRIMARY KEY,
                name     TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                data     BYTEA NOT NULL
            );",
        );

        self.execute(query).await.unwrap();

        let query = "CREATE TABLE IF NOT EXISTS sessions (
            token      TEXT NOT NULL PRIMARY KEY,
            expires_at TIMESTAMPTZ NOT NULL,
            user_id    INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )";

        self.execute(query).await.unwrap();
    }
}
