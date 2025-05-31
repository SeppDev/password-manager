#[macro_use]
extern crate rocket;

use cors::CORS;
use database::Database;

mod api;
mod cors;
mod database;
pub mod jwt;
mod tests;

use dotenv::dotenv;
use rocket::Config;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let port: u16 = match std::env::var("PORT") {
        Ok(p) => p.parse().unwrap(),
        Err(_) => 8000,
    };

    let db = loop {
        let db = match Database::open(&database_url).await {
            Ok(db) => db,
            Err(err) => {
                println!("Failed to open database: {err:?}");
                continue;
            }
        };
        match db.init_connection().await {
            Ok(_result) => break db,
            Err(e) => eprintln!("{e}"),
        };
    };

    let mut config = Config::default();
    config.port = port;

    rocket::build()
        .configure(config)
        .manage(db)
        .attach(CORS)
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
