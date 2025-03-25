use std::fmt::Display;

use super::Database;
use bcrypt::{DEFAULT_COST, hash, verify};
use sqlx::Executor;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub data: Vec<u8>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum LoginError {
    Database(String),
    UserNotFound,
    WrongPassword,
}
impl Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UserNotFound => "UserNotFound".into(),
                Self::WrongPassword => "WrongPassword".into(),
                _ => serde_json::to_string(self).unwrap(),
            }
        )
    }
}

pub const MAX_USERNAME_LENGTH: usize = 20;

impl Database {
    pub async fn create_account(&self, name: &String, password: &String) -> super::QueryResult {
        let hash = if cfg!(test) {
            hash(password, 4).unwrap()
        } else {
            hash(password, DEFAULT_COST).unwrap()
        };

        sqlx::query("INSERT INTO users(name, password, data) VALUES(?, ?, ?)")
            .bind(name)
            .bind(hash)
            .bind(String::new())
            .execute(&self.pool)
            .await
    }
    pub async fn login(&self, name: &String, password: &String) -> Result<String, LoginError> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE name = ?;")
            .bind(name)
            .fetch_one(&self.pool)
            .await;

        let user = match user {
            Ok(u) => u,
            Err(err) => {
                return Err(match err {
                    sqlx::Error::RowNotFound => LoginError::UserNotFound,
                    _ => LoginError::Database(err.to_string()),
                });
            }
        };

        let correct_password = verify(&password, &user.password).unwrap();
        if !correct_password {
            return Err(LoginError::WrongPassword);
        }

        let uuid = Uuid::new_v4().to_string();

        return Ok(uuid);
    }
    pub async fn fetch_users(&self) -> sqlx::Result<Vec<User>> {
        sqlx::query_as::<_, User>("SELECT * FROM users;")
            .fetch_all(&self.pool)
            .await
    }
    pub async fn get_user_by_name(&self, name: &String) -> sqlx::Result<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE name = ?;")
            .bind(name)
            .fetch_one(&self.pool)
            .await
    }
}
