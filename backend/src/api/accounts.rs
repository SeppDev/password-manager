
use rocket::{
    Request,
    http::Status,
    request::{FromRequest, Outcome},
};

use super::ApiResponse;

pub struct SingupCreds<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Debug)]
pub enum SignupError {
    InvalidHeaders,
    InvalidUsername,
    InvalidPassword
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

        if username.len() < 3 || username.len() > 20 {
            return ApiResponse::ok_message("Username must be between 3 and 20 characters long");
        } else if password.len() < 8 {
            return ApiResponse::ok_message("Password is too short");
        }

        Outcome::Success(SingupCreds { username, password })
    }
}