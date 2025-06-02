#[cfg(test)]
pub mod tests {
    use rand::{Rng, distr::Alphanumeric};
    use rocket::{http::Header, local::asynchronous::Client};

    use crate::{database::Database, server::build, tests::accounts::tests::AccountCreds};
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
    async fn unauthenticated() {
        let client = client().await;
        let response = client.get("/api/authenticated").dispatch().await;

        assert_eq!(response.status(), rocket::http::Status::Unauthorized);
    }

    #[sqlx_macros::test]
    async fn authenticated() {
        let client = client().await;
        let account = AccountCreds::random();

        let response = client
            .post("/api/signup")
            .header(Header::new("username", account.username))
            .header(Header::new("password", account.password))
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::Ok);
        let body = response.into_string().await.expect("Failed to get body");

        #[derive(serde::Deserialize)]
        struct Body {
            token: String,
        }

        let body = serde_json::from_str::<Body>(body.as_str()).expect("Failed to parse to json");
        let response = client
            .get("/api/authenticated")
            .header(Header::new("token", body.token))
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::Ok);
    }

    #[sqlx_macros::test]
    async fn user_no_exists() {
        let client = client().await;

        #[derive(serde::Deserialize)]
        struct Response<'a> {
            value: &'a str,
        }

        for _ in 0..100 {
            let creds = AccountCreds::random();
            let uri = format!("/api/user/exists/{}", creds.username);

            let response = client.get(uri.as_str()).dispatch().await;
            assert_eq!(response.status(), rocket::http::Status::Ok);

            let body = response.into_string().await.expect("Failed to get body");
            let body =
                serde_json::from_str::<Response>(body.as_str()).expect("Failed to parse to json");

            assert_eq!(body.value, "false");
        }
    }

    #[sqlx_macros::test]
    async fn user_exists() {
        let client = client().await;

        #[derive(serde::Deserialize)]
        struct Response<'a> {
            value: &'a str,
        }

        for _ in 0..100 {
            let account = AccountCreds::random().random_username(18);

            let response = client
                .post("/api/signup")
                .header(Header::new("username", account.username.clone()))
                .header(Header::new("password", account.password))
                .dispatch()
                .await;

            assert_eq!(response.status(), rocket::http::Status::Ok);

            let uri = format!("/api/user/exists/{}", account.username);

            let response = client.get(uri.as_str()).dispatch().await;
            assert_eq!(response.status(), rocket::http::Status::Ok);

            let body = response.into_string().await.expect("Failed to get body");
            let body =
                serde_json::from_str::<Response>(body.as_str()).expect("Failed to parse to json");

            assert_eq!(body.value, "true");
        }
    }

    #[sqlx_macros::test]
    async fn get_user_data() {
        let client = client().await;

        for _ in 0..100 {
            let account = AccountCreds::random().random_username(18);

            let response = client
                .post("/api/signup")
                .header(Header::new("username", account.username.clone()))
                .header(Header::new("password", account.password))
                .dispatch()
                .await;

            assert_eq!(response.status(), rocket::http::Status::Ok);

            #[derive(serde::Deserialize)]
            struct Body {
                token: String,
            }

            let body = response.into_string().await.expect("Failed to get body");
            let body =
                serde_json::from_str::<Body>(body.as_str()).expect("Failed to parse to json");

            let response = client
                .get("/api/userdata")
                .header(Header::new("token", body.token))
                .dispatch()
                .await;

            assert_eq!(response.status(), rocket::http::Status::Ok);
        }
    }

    #[sqlx_macros::test]
    async fn set_user_data() {
        let client = client().await;
        let account = AccountCreds::random();

        let response = client
            .post("/api/signup")
            .header(Header::new("username", account.username.clone()))
            .header(Header::new("password", account.password))
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::Ok);

        #[derive(serde::Deserialize)]
        struct Body {
            token: String,
        }

        let body = response.into_string().await.expect("Failed to get body");
        let body = serde_json::from_str::<Body>(body.as_str()).expect("Failed to parse to json");

        let token = body.token;
        let data = b"some user data";

        let response = client
            .post("/api/userdata")
            .header(Header::new("token", token))
            .body(data)
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::NoContent);
    }

    #[sqlx_macros::test]
    async fn get_user_data_random_token() {
        let client = client().await;
        let token: String = rand::rng()
            .sample_iter(Alphanumeric)
            .take(50)
            .map(char::from)
            .collect();

        let response = client
            .get("/api/userdata")
            .header(Header::new("token", token))
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::Unauthorized);
    }

    #[sqlx_macros::test]
    async fn get_user_data_unauthorized() {
        let client = client().await;

        let response = client.get("/api/userdata").dispatch().await;
        assert_eq!(response.status(), rocket::http::Status::BadRequest);
    }

    #[sqlx_macros::test]
    async fn create_account() {
        let client = client().await;
        let account = AccountCreds::random();

        let response = client
            .post("/api/signup")
            .header(Header::new("username", account.username))
            .header(Header::new("password", account.password))
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::Ok);
    }

    #[sqlx_macros::test]
    async fn login() {
        let client = client().await;
        let account = AccountCreds::random();

        let response = client
            .post("/api/signup")
            .header(Header::new("username", account.username.clone()))
            .header(Header::new("password", account.password.clone()))
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::Ok);
        let body = response.into_string().await.expect("Failed to get body");

        #[derive(serde::Deserialize)]
        #[allow(unused)]
        struct Body {
            token: String,
        }

        serde_json::from_str::<Body>(body.as_str()).expect("Failed to parse to json");

        let response = client
            .post("/api/login")
            .header(Header::new("username", account.username))
            .header(Header::new("password", account.password))
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::Ok);
    }

    #[sqlx_macros::test]
    async fn login_fail() {
        let client = client().await;
        let account = AccountCreds::random();

        let response = client
            .post("/api/login")
            .header(Header::new("username", account.username))
            .header(Header::new("password", account.password))
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::BadRequest);
    }

    #[sqlx_macros::test]
    async fn long_username() {
        let client = client().await;
        let account = AccountCreds::random().random_username(20);

        let response = client
            .post("/api/signup")
            .header(Header::new("username", account.username))
            .header(Header::new("password", account.password))
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::Ok);
    }

    #[sqlx_macros::test]
    async fn too_long_username() {
        let client = client().await;
        let account = AccountCreds::random().random_username(21);

        let response = client
            .post("/api/signup")
            .header(Header::new("username", account.username))
            .header(Header::new("password", account.password))
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::BadRequest);
    }

    #[sqlx_macros::test]
    async fn random_username() {
        let client = client().await;
        let account = AccountCreds::random().random_username(8);

        let response = client
            .post("/api/signup")
            .header(Header::new("username", account.username))
            .header(Header::new("password", account.password))
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::Ok);
    }

    #[sqlx_macros::test]
    async fn too_short_password() {
        let client = client().await;
        let account = AccountCreds::random().pass_len(2);

        let response = client
            .post("/api/signup")
            .header(Header::new("username", account.username))
            .header(Header::new("password", account.password))
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::BadRequest);
    }

    #[sqlx_macros::test]
    async fn long_password() {
        let client = client().await;
        let account = AccountCreds::random().pass_len(100);

        let response = client
            .post("/api/signup")
            .header(Header::new("username", account.username))
            .header(Header::new("password", account.password))
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::Ok);
    }

    #[sqlx_macros::test]
    async fn too_long_password() {
        let client = client().await;
        let account = AccountCreds::random().pass_len(101);

        let response = client
            .post("/api/signup")
            .header(Header::new("username", account.username))
            .header(Header::new("password", account.password))
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::BadRequest);
    }
}
