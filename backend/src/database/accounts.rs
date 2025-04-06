use std::fmt::Debug;

use chrono::{Duration, Utc};
use rand::{Rng, distr::Alphanumeric};
use sqlx::postgres::PgQueryResult;

use super::Database;
use bcrypt::{DEFAULT_COST, hash};

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub password: String,
}

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct UserSession {
    pub user_id: i64,
}

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct UserData {
    pub data: Vec<u8>,
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
    pub async fn create_session(&self, user_id: &i64) -> sqlx::Result<String> {
        let token: String = {
            let mut rng = rand::rng();
            (0..100).map(|_| rng.sample(Alphanumeric) as char).collect()
        };

        let expires_at = Utc::now() + Duration::days(1);
        let query =
            format!("INSERT INTO {SESSIONS_TABLE}(token, user_id, expires_at) VALUES($1, $2, $3)");

        sqlx::query(&query)
            .bind(&token)
            .bind(user_id)
            .bind(expires_at)
            .execute(&self.pool)
            .await?;

        return Ok(token);
    }
    pub async fn get_session(&self, token: &str) -> sqlx::Result<UserSession> {
        let query = format!("DELETE FROM {SESSIONS_TABLE} WHERE expires_at < NOW();");
        sqlx::query(&query)
            .execute(&self.pool)
            .await
            .expect("Failed to delete previous sessions");

        let query = format!("SELECT user_id FROM {SESSIONS_TABLE} WHERE token = ($1);");

        sqlx::query_as::<_, UserSession>(&query)
            .bind(token)
            .fetch_one(&self.pool)
            .await
    }
    pub async fn extend_session(&self, token: &str) -> sqlx::Result<()> {
        todo!()
    }
    pub async fn get_user_by_name(&self, name: &str) -> sqlx::Result<User> {
        let query = format!("SELECT id, name, password FROM {USERS_TABLE} WHERE name = ($1);");

        sqlx::query_as::<_, User>(&query)
            .bind(name.to_string())
            .fetch_one(&self.pool)
            .await
    }
    pub async fn get_user_by_id(&self, id: &i64) -> sqlx::Result<User> {
        let query = format!("SELECT * FROM {USERS_TABLE} WHERE id = ($1);");

        sqlx::query_as::<_, User>(&query)
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }
    pub async fn get_user_data(&self, id: &i64) -> sqlx::Result<UserData> {
        let query = format!("SELECT data FROM {USERS_TABLE} WHERE id = ($1);");

        sqlx::query_as::<_, UserData>(&query)
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }
    pub async fn set_user_data(&self, id: &i64, data: &str) -> sqlx::Result<()> {
        let query = format!("UPDATE {USERS_TABLE} SET data = ($1) WHERE id = ($2);");

        sqlx::query(&query)
            .bind(data)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
