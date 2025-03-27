use std::fmt::Debug;

use chrono::{Duration, Utc};
use sqlx::postgres::PgQueryResult;

use super::Database;
use bcrypt::{DEFAULT_COST, hash};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub password: String,
    pub data: Vec<u8>,
}

use super::db_config::{SESSIONS_TABLE, USERS_TABLE};

impl Database {
    pub async fn create_account(
        &self,
        name: impl ToString,
        password: impl ToString,
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
        let uuid = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + Duration::days(1);

        let query =
            format!("INSERT INTO {SESSIONS_TABLE}(token, user_id, expires_at) VALUES($1, $2, $3)");

        sqlx::query(&query)
            .bind(&uuid)
            .bind(user_id)
            .bind(expires_at)
            .execute(&self.pool)
            .await?;

        return Ok(uuid);
    }
    pub async fn is_token_valid(&self, token: &String) -> sqlx::Result<bool> {
        let query = format!("DELETE FROM {SESSIONS_TABLE} WHERE expires_at < NOW();");
        sqlx::query(&query)
            .execute(&self.pool)
            .await
            .expect("Failed to delete previous sessions");

        let query = format!("SELECT EXISTS(SELECT 1 FROM {SESSIONS_TABLE} WHERE token = $1);");
        let exists: bool = sqlx::query_scalar(&query)
            .bind(token)
            .fetch_one(&self.pool)
            .await
            .expect("Failed to select session");

        Ok(exists)
    }
    pub async fn get_user_by_name(&self, name: impl ToString) -> sqlx::Result<User> {
        let query = format!("SELECT * FROM {USERS_TABLE} WHERE name = $1;");

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
