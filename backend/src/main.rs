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

    let db = Database::open(&database_url).await.unwrap();
    db.init_connection().await;

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
