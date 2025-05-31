use sqlx::postgres::PgQueryResult;
use std::fmt::Debug;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

use super::config::{USERS_TABLE, VAULT_TABLE};
use super::{Database, UserClaims};

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct UserData {
    pub data: Option<Vec<u8>>,
}

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub password: String,
}

impl Database {
    pub async fn create_account(
        &self,
        name: &str,
        password: &str,
    ) -> super::QueryResult {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password = password.as_bytes();
        let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();

        let query = format!(
            "WITH new_user AS (
                INSERT INTO {USERS_TABLE} (name, password)
                VALUES ($1, $2)
                RETURNING id
            )
            INSERT INTO {VAULT_TABLE} (user_id)
            SELECT id FROM new_user;"
        );

        let query = sqlx::query(&query)
            .bind(name.to_string())
            .bind(password_hash);

        self.execute(query).await
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
        let query = sqlx::query_as(&query).bind(&name);
        self.fetch_one(query).await
    }
    pub async fn get_user_by_id(&self, id: i64) -> sqlx::Result<User> {
        let query = format!("SELECT * FROM {USERS_TABLE} WHERE id = ($1);");
        let query = sqlx::query_as(&query).bind(id);
        self.fetch_one(query).await
    }
    pub async fn get_user_data(&self, id: i64) -> sqlx::Result<UserData> {
        let query = format!("SELECT * FROM {VAULT_TABLE} WHERE user_id = ($1)");
        let query = sqlx::query_as::<_, UserData>(&query).bind(id);
        self.fetch_one(query).await
    }
    pub async fn set_user_data(&self, id: i64, data: &Vec<u8>) -> sqlx::Result<()> {
        let query = format!("UPDATE {VAULT_TABLE} SET data = ($1) WHERE user_id = ($2);");
        let query = sqlx::query(&query).bind(data).bind(id);
        self.execute(query).await?;
        Ok(())
    }
}
