use std::fmt::Debug;

use crate::api::ApiResponse;
use chrono::{Duration, Utc};

use super::Database;
use bcrypt::{DEFAULT_COST, hash, verify};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub password: String,
    pub data: String,
}

pub const MAX_USERNAME_LENGTH: usize = 20;

impl Database {
    pub async fn create_account(&self, name: &String, password: &String) -> super::QueryResult {
        let hash = if cfg!(test) || cfg!(debug_assertions) {
            hash(password, 4).unwrap()
        } else {
            hash(password, DEFAULT_COST).unwrap()
        };

        sqlx::query("INSERT INTO users(name, password, data) VALUES($1, $2, $3)")
            .bind(name)
            .bind(hash)
            .bind(String::new())
            .execute(&self.pool)
            .await
    }
    pub async fn login(&self, name: &String, password: &String) -> Result<String, ApiResponse> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE name = $1;")
            .bind(name)
            .fetch_one(&self.pool)
            .await;

        let user = match user {
            Ok(u) => u,
            Err(err) => {
                return Err(match err {
                    sqlx::Error::RowNotFound => ApiResponse::err_message("User not found"),
                    _ => ApiResponse::err_message(err.to_string()),
                });
            }
        };

        let correct_password = verify(&password, &user.password).unwrap();
        if !correct_password {
            return Err(ApiResponse::err_message("Wrong password"));
        }

        let uuid = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + Duration::days(1);

        let result =
            sqlx::query("INSERT INTO sessions(token, user_id, expires_at) VALUES($1, $2, $3)")
                .bind(&uuid)
                .bind(user.id)
                .bind(expires_at)
                .execute(&self.pool)
                .await;

        match result {
            Ok(_) => {}
            Err(err) => return Err(ApiResponse::err_message(err.to_string())),
        };

        return Ok(uuid);
    }
    pub async fn is_token_valid(&self, token: &String) -> sqlx::Result<bool> {
        sqlx::query("DELETE FROM sessions WHERE expires_at < NOW();")
            .execute(&self.pool)
            .await
            .expect("Failed to delete previous sessions");

        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM sessions WHERE token = $1);")
                .bind(token)
                .fetch_one(&self.pool)
                .await
                .expect("Failed to select session");

        Ok(exists)
    }
    pub async fn get_user_by_name(&self, name: &String) -> sqlx::Result<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE name = $1;")
            .bind(name)
            .fetch_one(&self.pool)
            .await
    }
    pub async fn get_user_by_id(&self, id: &i64) -> sqlx::Result<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE name = $1;")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }
}
