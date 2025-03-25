use sqlx::Executor;

use super::Database;

impl Database {
    pub async fn init_connection(&self) {
        self.pool
            .execute(
                "CREATE TABLE IF NOT EXISTS users (
                        id       BIGSERIAL PRIMARY KEY,
                        name     TEXT NOT NULL UNIQUE,
                        password TEXT NOT NULL,
                        data     TEXT NOT NULL
                    )",
            )
            .await
            .unwrap();

        self.pool
            .execute(
                "CREATE TABLE IF NOT EXISTS sessions (
                token      TEXT NOT NULL PRIMARY KEY,
                expires_at TIMESTAMP NOT NULL,
                user_id    INTEGER NOT NULL,
                FOREIGN KEY (user_id) REFERENCES users(id)
            )",
            )
            .await
            .unwrap();
    }
}
