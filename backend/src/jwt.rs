use std::marker::PhantomData;

use jwt_simple::{Error, claims::Claims, prelude::*};
use serde::de::DeserializeOwned;

type Key = Ed25519KeyPair;

pub struct JWTSession<T: Serialize + DeserializeOwned> {
    _type: PhantomData<T>,
    public_key: String,
    private_key: String,
}
impl<T: Serialize + DeserializeOwned> JWTSession<T> {
    pub fn new() -> Self {
        let key = Key::generate();
        let private_key = key.to_pem();
        let public_key = key.public_key().to_pem();

        Self {
            _type: PhantomData,
            public_key,
            private_key,
        }
    }
    pub fn create_session(&self, claims: T) -> Result<String, Error> {
        let key_pair = Key::from_pem(&self.private_key)?;
        let claims = Claims::with_custom_claims(claims, Duration::from_hours(12));
        key_pair.sign(claims)
    }
    pub fn get_claims(&self, token: &str) -> Result<T, Error> {
        let public_key = Ed25519PublicKey::from_pem(&self.public_key)?;
        let claims = public_key.verify_token::<T>(token, None)?;
        Ok(claims.custom)
    }
}
