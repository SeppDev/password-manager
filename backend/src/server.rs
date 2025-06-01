use rocket::{Build, Rocket};

use crate::{api, cors::CORS, database::Database};

use rocket::Config;

#[get("/status")]
async fn status() -> &'static str {
    "true"
}

pub async fn build(database: Database) -> Rocket<Build> {
    let port: u16 = match std::env::var("PORT") {
        Ok(p) => p.parse().unwrap(),
        Err(_) => 8000,
    };

    let mut config = Config::default();
    config.port = port;

    rocket::build()
        .configure(config)
        .manage(database)
        .attach(CORS)
        .mount("/", routes![status])
        .mount(
            "/api",
            routes![
                api::signup,
                api::login,
                api::user_data,
                api::authenticated,
                api::update_user_data,
                api::user_exists
            ],
        )
}
