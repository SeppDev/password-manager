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
use rocket::{
    Request, State,
    http::Status,
    request::{FromRequest, Outcome},
};
use serde::{Deserialize, Serialize};



pub struct SingupCreds<'a> {
    username: &'a str,
    password: &'a str,
}

#[derive(Debug)]
pub enum SignupError {
    InvalidHeaders,
    DuplicateUsername,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SingupCreds<'r> {
    type Error = SignupError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let headers = req.headers();
        let username = headers.get_one("username");
        let password = headers.get_one("password");

        let (username, password) = match (username, password) {
            (Some(n), Some(p)) => (n, p),
            _ => return Outcome::Error((Status::BadRequest, SignupError::InvalidHeaders)),
        };

        Outcome::Success(SingupCreds { username, password })
    }
}

#[derive(Serialize, Deserialize)]
enum ApiResponse {
    Message { message: &'static str },
}

#[post("/signup")]
pub async fn signup<'r>(
    db: &State<Database>,
    creds: SingupCreds<'r>,
) -> Result<&'static str, &'static str> {
    let result = db
        .create_account(creds.username.into(), creds.password.into())
        .await;

    if result.is_err() {
        return Err("Failed to create account");
    }

    return Ok("Successfully created account");
}

#[get("/login")]
pub async fn login<'r>(db: &State<Database>, creds: SingupCreds<'r>) -> String {
    let result = db
        .create_account(creds.username.into(), creds.password.into())
        .await;

    let response = ApiResponse::Message {
        message: "Successfully created account",
    };

    serde_json::to_string(&response).unwrap()
}

#[get("/users")]
pub async fn fetch_users(db: &State<Database>) -> String {
    let result = db.fetch_users().await;

    format!("{result:#?}")
}
