use bcrypt::DEFAULT_COST;
use serde::{Deserialize, Serialize};

use super::Database;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: usize,
    name: String,
    password: String,
}

impl Database {
    pub async fn create_account(&self, name: String, password: String) -> mysql_async::Result<()> {
        self.conn
            .call(move |conn| {
                let password_hash = bcrypt::hash(password, DEFAULT_COST).unwrap();

                conn.execute(
                    "INSERT INTO users (name, password) VALUES (?1, ?2)",
                    (&name, &password_hash),
                )?;

                Ok(())
            })
            .await
    }
    pub async fn fetch_users(&self) -> mysql_async::Result<Vec<User>> {
        self.conn
            .call(|conn| {
                let mut stmt = conn.prepare("SELECT id, name, password FROM users")?;
                let person_iter = stmt.query_map([], |row| {
                    Ok(User {
                        id: row.get(0).unwrap(),
                        name: row.get(1).unwrap(),
                        password: row.get(2).unwrap(),
                    })
                })?;

                let mut users = Vec::new();

                for user in person_iter {
                    users.push(user?);
                }

                Ok(users)
            })
            .await
    }
}
