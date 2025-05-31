#[cfg(test)]
mod tests {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    use fake::{Fake, faker};
    use rand::random;
    use sqlx::sqlx_macros;
    use std::env;

    use crate::database::Database;

    async fn create_database() -> anyhow::Result<Database> {
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("Expected database_url");
        let db = Database::test(database_url.as_str()).await?;
        db.init_connection()
            .await
            .expect("Failed connecting to database");

        Ok(db)
    }

    #[sqlx_macros::test]
    async fn it_connects() -> anyhow::Result<()> {
        create_database().await?;
        Ok(())
    }

    #[sqlx_macros::test]
    async fn create_account() {
        let db = create_database().await.unwrap();
        let username: String = faker::name::en::Name().fake();
        let password: String = random::<[char; 12]>().into_iter().collect();

        db.create_account(username.as_str(), password.as_str())
            .await
            .unwrap();
    }

    #[sqlx_macros::test]
    async fn password_hasing() {
        let db = create_database().await.unwrap();
        let username: String = faker::name::en::Name().fake();
        let password: String = random::<[char; 12]>().into_iter().collect();

        db.create_account(username.as_str(), password.as_str())
            .await
            .unwrap();

        let user = db.get_user_by_name(&username).await.unwrap();
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .expect("Password was incorrect");
    }

    #[sqlx_macros::test]
    async fn failed_login() {
        let db = create_database().await.unwrap();
        let username: String = faker::name::en::Name().fake();
        let password: String = random::<[char; 12]>().into_iter().collect();

        db.create_account(username.as_str(), password.as_str())
            .await
            .unwrap();

        let user = db.get_user_by_name(&username).await.unwrap();
        let parsed_hash = PasswordHash::new(&user.password).unwrap();

        let wrong_password: String = random::<[char; 12]>().into_iter().collect();

        Argon2::default()
            .verify_password(wrong_password.as_bytes(), &parsed_hash)
            .expect_err("Password was incorrecly verified");
    }
}
