use crate::database::Database;
use bcrypt::verify;
use rocket::State;

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

    #[response(status = 400)]
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
        Self::Err(Self::key_value(key, value))
    }
    pub fn ok_message(body: impl ToString) -> ApiResponse {
        Self::Ok(Self::message_string(body))
    }
    pub fn err_message(body: impl ToString) -> ApiResponse {
        Self::Err(Self::message_string(body))
    }
}

#[post("/signup")]
pub async fn signup<'r>(db: &State<Database>, creds: SingupCreds<'r>) -> ApiResponse {

    match db.get_user_by_name(&creds.username).await {
        Ok(_) => return ApiResponse::ok_message("Account already exists"),
        Err(err) => match err {
            sqlx::Error::RowNotFound => {},
            _ => return ApiResponse::ok_message(err.to_string())
        }
    };

    let _result = match db.create_account(&creds.username, creds.password).await {
        Ok(r) => r,
        Err(err) => return ApiResponse::err_message(err.to_string())
    };

    let user = match db.get_user_by_name(&creds.username).await {
        Ok(u) => u,
        Err(err) => return ApiResponse::err_message(err.to_string())
    };

    let token = match db.create_token(&user.id).await {
        Ok(t) => t,
        Err(_) => return ApiResponse::err_message("Failed to create session token")
    };

    let json = json!({"message": "Succesfully created account", "token": token});

    ApiResponse::Ok(json.to_string())
}

#[post("/login")]
pub async fn login<'r>(db: &State<Database>, creds: SingupCreds<'r>) -> ApiResponse {
    let user = db.get_user_by_name(&creds.username.to_string()).await;
    let user = match user {
        Ok(u) => u,
        Err(err) => {
            return match err {
                sqlx::Error::RowNotFound => ApiResponse::ok_message("User not found"),
                _ => ApiResponse::err_message(err.to_string()),
            };
        }
    };

    let correct_password = verify(&creds.password, &user.password).unwrap();
    if !correct_password {
        return ApiResponse::ok_message("Wrong password");
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
