use std::{collections::BTreeMap, fmt::Debug};

use chrono::{Duration, Utc};
use rand::{Rng, distr::Alphanumeric};
use sqlx::postgres::PgQueryResult;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

use super::db_config::USERS_TABLE;
use super::{Database, UserClaims};

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub password: String,
}

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct UserData {
    pub data: Vec<u8>,
}

impl Database {
    pub async fn create_account(
        &self,
        name: &str,
        password: &str,
    ) -> super::QueryResult<PgQueryResult> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password = password.as_bytes();
        let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();

        let query = format!("INSERT INTO {USERS_TABLE} (name, password, data) VALUES($1, $2, $3)");
        sqlx::query(&query)
            .bind(name.to_string())
            .bind(password_hash)
            .bind(Vec::new() as Vec<u8>)
            .execute(&self.pool)
            .await
    }
    pub fn create_session(&self, user_id: i64) -> String {
        self.jwt.create_session(UserClaims { user_id }).unwrap()
    }
    pub fn get_session(&self, token: &str) -> Option<UserClaims> {
        match self.jwt.get_claims(token) {
            Ok(c) => Some(c),
            Err(..) => None,
        }
    }
    pub async fn get_user_by_name(&self, name: &str) -> sqlx::Result<User> {
        let query = format!(
            "SELECT id, name, password FROM {USERS_TABLE} WHERE LOWER(name) = LOWER(($1));"
        );

        sqlx::query_as::<_, User>(&query)
            .bind(name.to_string())
            .fetch_one(&self.pool)
            .await
    }
    pub async fn get_user_by_id(&self, id: i64) -> sqlx::Result<User> {
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
    pub async fn set_user_data(&self, id: &i64, data: &Vec<u8>) -> sqlx::Result<()> {
        let query = format!("UPDATE {USERS_TABLE} SET data = ($1) WHERE id = ($2);");

        sqlx::query(&query)
            .bind(data)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
