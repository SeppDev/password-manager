use crate::database::{Database, UserClaims, accounts::User};
use base64::{Engine, prelude::BASE64_STANDARD};
use rocket::State;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};

mod accounts;
use accounts::{SignupCreds, UserData};
use serde_json::json;

mod response;
use response::{ApiResponse, ApiResult};

// impl<T> Into<sqlx::Result<T>> for sqlx::Result<T, ApiResponse> {
//     fn into(self) -> sqlx::Result<T> {
//         todo!()
//     }
// }

#[post("/signup")]
pub async fn signup<'r>(db: &State<Database>, creds: SignupCreds<'r>) -> ApiResult {
    let (username, password) = creds.may_fail()?;

    match db.get_user_by_name(&username).await {
        Ok(_) => return ApiResponse::ok_message("Account already exists").into(),
        Err(err) => match err {
            sqlx::Error::RowNotFound => {}
            _ => return ApiResponse::ok_message(err.to_string()).into(),
        },
    };

    let _result = match db.create_account(&username, password).await {
        Ok(r) => r,
        Err(err) => return ApiResponse::err_message(err.to_string()).into(),
    };

    let user = match db.get_user_by_name(&username).await {
        Ok(u) => u,
        Err(err) => return ApiResponse::err_message(err.to_string()).into(),
    };

    let token = db.create_session(user.id);
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
            }
            .into();
        }
    };

    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    let correct_password = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    if !correct_password {
        return ApiResponse::ok_message("Wrong password").into();
    }

    let token = db.create_session(user.id);
    ApiResponse::ok_key_value("token", token).into()
}

#[get("/userdata")]
pub async fn user_data<'r>(db: &State<Database>, user: User) -> String {
    let vault = db.get_user_data(user.id).await.unwrap();
    BASE64_STANDARD.encode(vault.data.unwrap_or_default())
}

#[post("/userdata")]
pub async fn update_user_data<'r>(
    db: &State<Database>,
    user: User,
    input: UserData,
) -> ApiResponse {
    let bytes = BASE64_STANDARD.decode(input.0).unwrap();
    db.set_user_data(user.id, &bytes).await.unwrap();

    ApiResponse::NoContent(())
}

#[get("/user/exists/<username>")]
pub async fn user_exists<'r>(db: &State<Database>, username: &str) -> ApiResponse {
    if !username.is_ascii() {
        return ApiResponse::ok_message("Username must only contain valid ascii characters");
    }

    match db.get_user_by_name(username).await {
        Ok(_) => return ApiResponse::ok_key_value("value", "true"),
        Err(err) => {
            return match err {
                sqlx::Error::RowNotFound => ApiResponse::ok_key_value("value", "false"),
                _ => ApiResponse::Err(format!("{err}")),
            };
        }
    }
}

#[get("/authenticated")]
pub async fn authenticated<'r>(_user: UserClaims) {}
