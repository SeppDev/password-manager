#[cfg(test)]
pub mod tests {
    use crate::database::Database;

    use fake::{Fake, faker};
    use rand::{Rng, distr::Alphanumeric, random};
    use sqlx::sqlx_macros;

    pub struct AccountCreds {
        pub username: String,
        pub password: String,
    }
    impl AccountCreds {
        pub fn random() -> Self {
            let firstname: String = faker::name::en::FirstName().fake();
            let username = firstname.to_ascii_lowercase();
            let password = String::with_capacity(1);
            return Self { username, password }.pass_len(12);
        }
        pub fn random_username(mut self, len: usize) -> Self {
            let username = rand::rng()
                .sample_iter(Alphanumeric)
                .take(len)
                .map(char::from)
                .collect();

            self.username = username;
            self
        }
        pub fn pass_len(mut self, len: usize) -> Self {
            let password = rand::rng()
                .sample_iter(Alphanumeric)
                .take(len)
                .map(char::from)
                .collect();

            self.password = password;
            self
        }
    }

    impl Database {
        pub async fn create_fake_account(&mut self) -> AccountCreds {
            let creds = AccountCreds::random();

            self.create_account(&creds.username, &creds.password)
                .await
                .unwrap();

            return creds;
        }
    }

    #[sqlx_macros::test]
    async fn account_creation() {
        let mut db = Database::test().await;
        db.create_fake_account().await;
    }

    #[sqlx_macros::test]
    async fn multiple_accounts_creation() {
        let db = Database::test().await;
        for _ in 0..100 {
            let creds = AccountCreds::random().random_username(20);
            db.create_account(&creds.username, &creds.password)
                .await
                .unwrap();

            let user = db
                .get_user_by_name(&creds.username)
                .await
                .expect("Failed to find user");

            db.get_user_data(user.id)
                .await
                .expect("Failed to get userdata");
        }
    }

    #[sqlx_macros::test]
    async fn account_fetching() {
        let mut db = Database::test().await;
        let account = db.create_fake_account().await;
        db.get_user_by_name(&account.username)
            .await
            .expect("Account was not found");
    }

    #[sqlx_macros::test]
    async fn account_fetching_fail() {
        let db = Database::test().await;
        let username: String = faker::name::en::Name().fake();

        db.get_user_by_name(&username)
            .await
            .expect_err("Account was found?");
    }

    #[sqlx_macros::test]
    async fn duplicate_account() {
        let db = Database::test().await;
        let username: String = faker::name::en::Name().fake();
        let password: String = random::<[char; 12]>().into_iter().collect();

        db.create_account(&username, &password).await.unwrap();

        db.create_account(&username, &password).await.unwrap_err();
    }

    #[sqlx_macros::test]
    async fn fetch_vault() {
        let mut db = Database::test().await;
        let creds = db.create_fake_account().await;
        let account = db
            .get_user_by_name(&creds.username)
            .await
            .expect("Failed to fetch user");
        db.get_user_data(account.id)
            .await
            .expect("Failed to fetch userdata");
    }

    #[sqlx_macros::test]
    async fn fetch_invalid_vault() {
        let db = Database::test().await;
        use rand::Rng;
        let user_id: i64 = rand::rng().random_range(0..i64::MAX);

        db.get_user_data(user_id)
            .await
            .expect_err("fetched userdata");
    }
}
