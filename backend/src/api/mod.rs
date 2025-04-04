use crate::database::{accounts::User, Database};
use bcrypt::verify;
use rocket::State;

mod accounts;
use accounts::SignupCreds;
use serde_json::json;

mod response;
use response::{ApiResponse, ApiResult};

#[derive(Default)]
pub(crate) enum ContentLanguage {
    #[default]
    English,
    Dutch,
}



#[post("/signup")]
pub async fn signup<'r>(db: &State<Database>, creds: SignupCreds<'r>) -> ApiResult {
    let (username, password) = creds.may_fail()?;

    match db.get_user_by_name(&username).await {
        Ok(_) => return ApiResponse::ok_message("Account already exists").into(),
        Err(err) => match err {
            sqlx::Error::RowNotFound => {},
            _ => return ApiResponse::ok_message(err.to_string()).into()
        }
    };

    let _result = match db.create_account(&username, password).await {
        Ok(r) => r,
        Err(err) => return ApiResponse::err_message(err.to_string()).into()
    };

    let user = match db.get_user_by_name(&username).await {
        Ok(u) => u,
        Err(err) => return ApiResponse::err_message(err.to_string()).into()
    };

    let token = match db.create_token(&user.id).await {
        Ok(t) => t,
        Err(_) => return ApiResponse::err_message("Failed to create session token").into()
    };      

    let json = json!({"token": token});

    ApiResponse::Ok(json.to_string()).into()
}

#[post("/login")]
pub async fn login<'r>(db: &State<Database>, creds: SignupCreds<'r>) -> ApiResult {
    let (username, password) = creds.may_fail()?;

    let user = db.get_user_by_name(&username).await;
    let user = match user {
        Ok(u) => u,
        Err(err) => {
            return match err {
                sqlx::Error::RowNotFound => ApiResponse::ok_message("User not found"),
                _ => ApiResponse::err_message(err.to_string()),
            }.into();
        }
    };

    let correct_password = verify(&password, &user.password).unwrap();
    if !correct_password {
        return ApiResponse::ok_message("Wrong password").into();
    }

    let token = db.create_token(&user.id).await.unwrap();
    ApiResponse::ok_key_value("token", token).into()
}



#[get("/userinfo")]
pub async fn user_info<'r>( user: User) -> String {
    serde_json::to_string(&user).unwrap()
}