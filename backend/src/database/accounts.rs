use bcrypt::{DEFAULT_COST, hash};

use super::Database;

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct User {
    id: i32,
    name: String,
    password: String,
    data: Vec<u8>,
}

impl Database {
    pub async fn create_account(&self, name: String, password: String) -> super::QueryResult {
        let hash = hash(password, DEFAULT_COST).unwrap();

        sqlx::query("INSERT INTO users (name, password, data) VALUES (?, ?, ?)")
            .bind(name)
            .bind(hash)
            .bind(Vec::new())
            .execute(&self.pool)
            .await
    }
    pub async fn fetch_users(&self) -> sqlx::Result<Vec<User>> {
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
    }
}
