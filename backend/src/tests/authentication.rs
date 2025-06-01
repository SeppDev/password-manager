#[cfg(test)]
pub mod tests {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    use rand::random;
    use sqlx::sqlx_macros;

    use crate::{
        database::{Database, UserClaims},
        jwt::JWTSession,
    };

    #[sqlx_macros::test]
    async fn password_hashing() {
        let mut db = Database::test().await;

        for _ in 0..10 {
            let account = db.create_fake_account().await;

            let user = db.get_user_by_name(&account.username).await.unwrap();
            let parsed_hash = PasswordHash::new(&user.password).unwrap();
            Argon2::default()
                .verify_password(account.password.as_bytes(), &parsed_hash)
                .expect("Password was incorrect");
        }
    }

    #[sqlx_macros::test]
    async fn wrong_password() {
        let mut db = Database::test().await;

        for _ in 0..10 {
            let account = db.create_fake_account().await;

            let user = db.get_user_by_name(&account.username).await.unwrap();
            let parsed_hash = PasswordHash::new(&user.password).unwrap();

            let wrong_password: String = random::<[char; 12]>().into_iter().collect();

            Argon2::default()
                .verify_password(wrong_password.as_bytes(), &parsed_hash)
                .expect_err("Password was incorrecly verified");
        }
    }

    #[sqlx_macros::test]
    async fn jwt_verification() {
        let jwt: JWTSession<UserClaims> = JWTSession::new();

        use rand::Rng;
        let user_id: i64 = rand::rng().random_range(0..i64::MAX);

        let token = jwt
            .create_token(UserClaims { user_id })
            .expect("Failed to create token");

        let claims = jwt.get_claims(&token).expect("No userclaims");
        assert!(user_id == claims.user_id)
    }

    #[sqlx_macros::test]
    async fn invalid_jwt() {
        let jwt: JWTSession<UserClaims> = JWTSession::new();

        use rand::Rng;
        let user_id: i64 = rand::rng().random_range(0..i64::MAX);

        let token = jwt
            .create_token(UserClaims { user_id })
            .expect("Failed to create token");

        let user_id: i64 = rand::rng().random_range(0..i64::MAX);

        let claims = jwt.get_claims(&token).expect("No userclaims");
        assert!(user_id != claims.user_id)
    }

    #[sqlx_macros::test]
    async fn tampered_jwt() {
        let jwt: JWTSession<UserClaims> = JWTSession::new();

        use rand::Rng;
        let user_id: i64 = rand::rng().random_range(0..i64::MAX);

        let mut token = jwt
            .create_token(UserClaims { user_id })
            .expect("Failed to create token");

        token.truncate(token.len() - 1);

        jwt.get_claims(&token).expect_err("Got userclaims");
    }
}
