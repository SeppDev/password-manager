
use rocket::{
    Request,
    http::Status,
    request::{FromRequest, Outcome},
};

pub struct SingupCreds<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Debug)]
pub enum SignupError {
    InvalidHeaders,
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