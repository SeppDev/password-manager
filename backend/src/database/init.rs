use super::Database;
use super::config::{USERS_TABLE, VAULT_TABLE};

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
        self._init_connection(
            temp_prefix(USERS_TABLE).as_str(),
            temp_prefix(VAULT_TABLE).as_str(),
        )
        .await
    }

    pub async fn init_connection(&self) {
        self._init_connection(USERS_TABLE, VAULT_TABLE).await;
    }

    async fn _init_connection(&self, users: &str, vault: &str) {
        let query = format!(
            "
            CREATE TABLE IF NOT EXISTS {users} (
                id         BIGSERIAL PRIMARY KEY,
                name       TEXT NOT NULL UNIQUE,
                password   TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS {vault} (
              id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
              user_id BIGINT REFERENCES {users}(id) ON DELETE CASCADE,
              updated_at TIMESTAMPTZ DEFAULT now(),
              data BYTEA
            );

            ",
        );

        self.execute(query).await.unwrap();
    }
}
