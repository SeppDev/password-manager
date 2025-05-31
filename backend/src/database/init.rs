use super::Database;
use super::config::{USERS_TABLE, VAULT_TABLE};
use super::db::QueryResult;

impl Database {
    pub async fn init_connection(&self) -> QueryResult {
        let query = format!(
            "
            CREATE TABLE IF NOT EXISTS {USERS_TABLE} (
                id         BIGSERIAL PRIMARY KEY,
                name       TEXT NOT NULL UNIQUE,
                password   TEXT NOT NULL
            );


            ",
        );

        self.query(&query).await?;

        let query = format!(
            "
            CREATE TABLE IF NOT EXISTS {VAULT_TABLE} (
              id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
              user_id BIGINT REFERENCES {USERS_TABLE}(id) ON DELETE CASCADE,
              data BYTEA
            );
            "
        );

        self.query(&query).await
    }
}
