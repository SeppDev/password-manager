
use crate::database::{accounts::LoginError, Database};
use rocket::{
    Request, State,
    http::Status,
    request::{FromRequest, Outcome},
};
use serde::{Deserialize, Serialize};

mod accounts;
use accounts::{SignupError, SingupCreds};


#[derive(Serialize, Deserialize)]
enum ApiResponse {
    Message(String),
    Token(String)
}

#[post("/signup")]
pub async fn signup<'r>(
    db: &State<Database>,
    creds: SingupCreds<'r>,
) -> String {
    let result = db
        .create_account(&creds.username.into(), &creds.password.into())
        .await;

    let message = match result {
        Ok(_) => "Succesfully created account",
        Err(_) => "Failed to create account"
    };
    
    serde_json::to_string(&ApiResponse::Message(message.into())).unwrap()
}

#[get("/login")]
pub async fn login<'r>(db: &State<Database>, creds: SingupCreds<'r>) -> Result<String, String> {
    let result = db.login(&creds.username.into(), &creds.password.into()).await;

    let response = match result {
        Ok(t) => ApiResponse::Token(t),
        Err(err) => ApiResponse::Message(err.to_string()),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/users")]
pub async fn fetch_users(db: &State<Database>) -> String {
    let result = db.fetch_users().await.unwrap();

    serde_json::to_string(&result).unwrap()
}
