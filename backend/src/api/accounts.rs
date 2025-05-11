use rocket::{
    Request,
    http::Status,
    request::{FromRequest, Outcome},
};

use crate::database::{Database, UserClaims, accounts::User};
use rocket::State;

use super::ApiResponse;

pub enum SignupCreds<'a> {
    Info {
        username: &'a str,
        password: &'a str,
    },
    Err(ApiResponse),
}
impl<'a> SignupCreds<'a> {
    pub fn may_fail(self) -> Result<(&'a str, &'a str), ApiResponse> {
        match self {
            Self::Info { username, password } => Ok((username, password)),
            Self::Err(e) => return Err(e),
        }
    }
}

#[derive(Debug)]
pub enum SignupError {
    MissingHeaders,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SignupCreds<'r> {
    type Error = SignupError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let headers = req.headers();
        let username = headers.get_one("username");
        let password = headers.get_one("password");

        let (username, password) = match (username, password) {
            (Some(n), Some(p)) => (n, p),
            _ => return Outcome::Error((Status::BadRequest, SignupError::MissingHeaders)),
        };

        if !username.is_ascii() {
            return Outcome::Success(Self::Err(ApiResponse::ok_message(
                "Username must only contain valid ascii characters",
            )));
        }

        if username.len() < 3 || username.len() > 20 {
            return Outcome::Success(Self::Err(ApiResponse::ok_message(
                "Username must be between 3 and 20 characters long",
            )));
        } else if password.len() < 8 {
            return Outcome::Success(Self::Err(ApiResponse::ok_message("Password is too short")));
        }

        Outcome::Success(Self::Info { username, password })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let db = match req.guard::<&State<Database>>().await {
            Outcome::Success(db) => db,
            Outcome::Forward(status) | Outcome::Error((status, _)) => {
                return Outcome::Error((status, ()));
            }
        };

        let token = match req.headers().get("token").next() {
            Some(t) => t,
            None => return Outcome::Error((Status::BadRequest, ())),
        };
        let session = match db.get_session(token) {
            Some(s) => s,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        let user = match db.get_user_by_id(session.user_id).await {
            Ok(u) => u,
            Err(_) => return Outcome::Error((Status::InternalServerError, ())),
        };

        Outcome::Success(user)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserClaims {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let db = match req.guard::<&State<Database>>().await {
            Outcome::Success(db) => db,
            Outcome::Forward(status) | Outcome::Error((status, _)) => {
                return Outcome::Error((status, ()));
            }
        };
        let token = match req.headers().get("token").next() {
            Some(t) => t,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };
        let session = match db.get_session(token) {
            Some(s) => s,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        Outcome::Success(session)
    }
}

pub struct UserData(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserData {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get("data").next() {
            Some(t) => Outcome::Success(UserData(t.to_string())),
            None => Outcome::Error((Status::InternalServerError, ())),
        }
    }
}
