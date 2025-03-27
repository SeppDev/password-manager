use crate::database::Database;
use bcrypt::verify;
use rocket::{State, http::Status};

mod accounts;
use accounts::SingupCreds;
use serde_json::json;

#[derive(Default)]
pub(crate) enum ContentLanguage {
    #[default]
    English,
    Dutch,
}

#[derive(Responder)]
pub(crate) enum ApiResponse {
    #[response(status = 204)]
    NoContent(()),

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
    fn key_value(key: impl ToString, value: impl ToString) -> String {
        let key = key.to_string();
        let value = value.to_string();
        let value = json!({key: value});
        serde_json::to_string(&value).unwrap()
    }
    pub fn ok_key_value(key: impl ToString, value: impl ToString) -> ApiResponse {
        Self::Ok(Self::key_value(key, value))
    }
    pub fn err_key_value(key: impl ToString, value: impl ToString) -> ApiResponse {
        Self::Ok(Self::key_value(key, value))
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
    let result = db.create_account(&creds.username, creds.password).await;

    if result.is_ok() {
        let user = db.get_user_by_name(creds.username).await.unwrap();
        let token = db.create_token(&user.id).await.unwrap();
        let json = json!({"message": "Succesfully created account", "token": token});
        return ApiResponse::Ok(json.to_string());
    }

    ApiResponse::err_message("Failed to create account")
}

#[get("/login")]
pub async fn login<'r>(db: &State<Database>, creds: SingupCreds<'r>) -> ApiResponse {
    let user = db.get_user_by_name(&creds.username.to_string()).await;
    let user = match user {
        Ok(u) => u,
        Err(err) => {
            return match err {
                sqlx::Error::RowNotFound => ApiResponse::err_message("User not found"),
                _ => ApiResponse::err_message(err.to_string()),
            };
        }
    };

    let correct_password = verify(&creds.password, &user.password).unwrap();
    if !correct_password {
        return ApiResponse::err_message("Wrong password");
    }

    let token = db.create_token(&user.id).await.unwrap();
    ApiResponse::ok_key_value("token", token)
}

#[get("/valid_token/<token>")]
pub async fn valid_token<'r>(db: &State<Database>, token: &'r str) -> ApiResponse {
    let result = db.is_token_valid(&token.into()).await;

    match result {
        Ok(bool) => ApiResponse::ok_message(format!("{bool}")),
        Err(_) => ApiResponse::err_message("Failed to verify token"),
    }
}
