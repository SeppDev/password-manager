use std::fmt::Debug;

use base64::{Engine, prelude::BASE64_STANDARD};
use chrono::{Duration, Utc};
use rand::seq::SliceRandom;
use sqlx::postgres::PgQueryResult;

use super::Database;
use bcrypt::{DEFAULT_COST, hash};

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub password: String,
    pub data: Vec<u8>,
}

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct Session {
    pub user_id: i64,
}

use super::db_config::{SESSIONS_TABLE, USERS_TABLE};

impl Database {
    pub async fn create_account(
        &self,
        name: &str,
        password: &str,
    ) -> super::QueryResult<PgQueryResult> {
        let hash = hash(password.to_string(), DEFAULT_COST).unwrap();

        let query = format!("INSERT INTO {USERS_TABLE} (name, password, data) VALUES($1, $2, $3)");
        sqlx::query(&query)
            .bind(name.to_string())
            .bind(hash)
            .bind(Vec::new() as Vec<u8>)
            .execute(&self.pool)
            .await
    }
    pub async fn create_token(&self, user_id: &i64) -> sqlx::Result<String> {
        let bytes: Vec<u8> = rand::random_iter().take(100   ).collect();
        let expires_at = Utc::now() + Duration::days(1);

        let query =
            format!("INSERT INTO {SESSIONS_TABLE}(token, user_id, expires_at) VALUES($1, $2, $3)");

        sqlx::query(&query)
            .bind(&bytes)
            .bind(user_id)
            .bind(expires_at)
            .execute(&self.pool)
            .await?;

        let token = BASE64_STANDARD.encode(bytes);
        return Ok(token);
    }
    pub async fn get_token_session(&self, token: &str) -> sqlx::Result<Session> {
        let query = format!("DELETE FROM {SESSIONS_TABLE} WHERE expires_at < NOW();");
        sqlx::query(&query)
            .execute(&self.pool)
            .await
            .expect("Failed to delete previous sessions");

        let query =
            format!("SELECT EXISTS(SELECT user_id FROM {SESSIONS_TABLE} WHERE token = $1);");
        sqlx::query_as::<_, Session>(&query)
            .bind(token)
            .fetch_one(&self.pool)
            .await
    }
    pub async fn extend_token(&self, token: &str) -> sqlx::Result<()> {
        todo!()
    }
    pub async fn get_user_by_name(&self, name: &str) -> sqlx::Result<User> {
        let query = format!("SELECT * FROM {USERS_TABLE} WHERE name = ($1);");

        sqlx::query_as::<_, User>(&query)
            .bind(name.to_string())
            .fetch_one(&self.pool)
            .await
    }
    pub async fn get_user_by_id(&self, id: &i64) -> sqlx::Result<User> {
        let query = format!("SELECT * FROM {USERS_TABLE} WHERE id = $1;");

        sqlx::query_as::<_, User>(&query)
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }
}
