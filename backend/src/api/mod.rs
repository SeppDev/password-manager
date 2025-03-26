use crate::database::Database;
use rocket::State;

mod accounts;
use accounts::SingupCreds;
use serde_json::json;

#[derive(Responder)]
pub(crate) enum ApiResponse {
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

    match result {
        Ok(_) => ApiResponse::ok_message("Succesfully created account"),
        Err(_) => ApiResponse::err_message("Failed to create account"),
    }
}

#[get("/login")]
pub async fn login<'r>(
    db: &State<Database>,
    creds: SingupCreds<'r>,
) -> Result<String, ApiResponse> {
    db.login(&creds.username.into(), &creds.password.into())
        .await
}

#[get("/valid_token/<token>")]
pub async fn valid_token<'r>(db: &State<Database>, token: &'r str) -> ApiResponse {
    let result = db.is_token_valid(&token.into()).await;

    match result {
        Ok(bool) => ApiResponse::ok_message(format!("{bool}")),
        Err(_) => ApiResponse::err_message("Failed to verify token"),
    }
}
