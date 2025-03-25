use crate::database::Database;
use rocket::State;
use serde::{Deserialize, Serialize};

mod accounts;
use accounts::SingupCreds;
use serde_json::json;

#[derive(Responder)]
enum ApiResponse {
    #[response(status = 200)]
    Ok(String),

    #[response(status = 404)]
    Err(String),
}
impl ApiResponse {
    fn message_string(body: impl ToString) -> String {
        let message = body.to_string();
        let value = json!({"message": message});
        serde_json::to_string(&value).unwrap()
    }
    fn key_value(key: impl ToString, body: impl ToString) -> String {
        let key = key.to_string();
        let body = body.to_string();
        let value = json!({key: body});
        serde_json::to_string(&value).unwrap()
    }
    pub fn ok_key_value(body: impl ToString) -> ApiResponse {
        Self::Ok(Self::message_string(body))
    }
    pub fn err_key_value(body: impl ToString) -> ApiResponse {
        Self::Ok(Self::message_string(body))
    }
    pub fn ok_message(body: impl ToString) -> ApiResponse {
        Self::Ok(Self::message_string(body))
    }
    pub fn err_message(body: impl ToString) -> ApiResponse {
        Self::Ok(Self::message_string(body))
    }
}

#[post("/signup")]
pub async fn signup<'r>(db: &State<Database>, creds: SingupCreds<'r>) -> ApiResponse {
    let result = db
        .create_account(&creds.username.into(), &creds.password.into())
        .await;

    println!("{result:#?}");

    match result {
        Ok(_) => ApiResponse::ok_message("Succesfully created account"),
        Err(_) => ApiResponse::err_message("Failed to create account"),
    }
}

#[get("/login")]
pub async fn login<'r>(db: &State<Database>, creds: SingupCreds<'r>) -> ApiResponse {
    let result = db
        .login(&creds.username.into(), &creds.password.into())
        .await;

    match result {
        Ok(t) => ApiResponse::ok_message(t),
        Err(err) => ApiResponse::err_message(err.to_string()),
    }
}

#[get("/users")]
pub async fn fetch_users(db: &State<Database>) -> ApiResponse {
    let result = db.fetch_users().await.unwrap();
    ApiResponse::Ok(serde_json::to_string(&result).unwrap())
}
