// use rocket::Request;
// use serde::{Deserialize, Serialize};
// use tokio_rusqlite::Connection;

// #[derive(Serialize, Deserialize)]
// struct User {
//     name: String,
//     age: u8,
//     alive: bool
// }

use crate::database::Database;
use bcrypt::{DEFAULT_COST, hash};
use rocket::{Request, State, request::FromRequest};
use serde::{Deserialize, Serialize};
use tokio_rusqlite::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: usize,
    name: String,
    password: String,
}

impl Database {
    async fn create_account(&self, name: String, password: String) -> Result<()> {
        self.conn
            .call(move |conn| {
                let password_hash = hash(password, DEFAULT_COST).unwrap();

                conn.execute(
                    "INSERT INTO users (name, password) VALUES (?1, ?2)",
                    (&name, &password_hash),
                )?;

                Ok(())
            })
            .await
    }
    async fn fetch_users(&self) -> Result<Vec<User>> {
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

                // let users = person_iter.collect();

                let mut users = Vec::new();

                for user in person_iter {
                    // println!("Found person {:?}", person?);
                    users.push(user?);
                }

                Ok(users)
            })
            .await
    }
}

enum SignupState {
    Creds { username: String, password: String },
    None,
}
impl<'a, 'r> FromRequest<'a> for SignupState {
    type Error = ();

    
}

#[get("/signup")]
pub async fn signup<'r>(db: &State<Database>, state: SignupState) -> String {
    let result = db
        .create_account("cool".to_string(), "password123".to_string())
        .await;

    format!("Result: {result:#?}")
}

#[get("/users")]
pub async fn fetch_users(db: &State<Database>) -> String {
    let result = db.fetch_users().await;

    format!("{result:#?}")
}
