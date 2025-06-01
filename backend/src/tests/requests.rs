#[cfg(test)]
pub mod tests {
    use rocket::{http::Header, local::asynchronous::Client};

    use crate::{database::Database, server::build};
    use rand::random;
    use sqlx::sqlx_macros;

    async fn client() -> Client {
        let db = Database::test().await;
        let build = build(db).await;
        Client::tracked(build)
            .await
            .expect("Failed to build client")
    }

    #[sqlx_macros::test]
    async fn status() {
        let client = client().await;
        let response = client.get("/status").dispatch().await;

        assert_eq!(response.status(), rocket::http::Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), "true");
    }

    #[sqlx_macros::test]
    async fn create_account() {
        let client = client().await;

        let username: String = random::<[char; 18]>().into_iter().collect();
        let password: String = random::<[char; 12]>().into_iter().collect();

        let response = client
            .post("/api/signup")
            .header(Header::new("username", username))
            .header(Header::new("password", password))
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::Ok);
    }
}
