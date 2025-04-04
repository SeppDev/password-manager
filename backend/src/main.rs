#[macro_use]
extern crate rocket;

use cors::CORS;
use database::Database;

mod api;
mod cors;
mod database;
mod tests;

use dotenv::dotenv;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");

    let db = Database::open(&database_url).await.unwrap();
    db.init_connection().await;

    rocket::build()
    .manage(db)
    .attach(CORS)
        .mount("/api", routes![api::signup, api::login, api::user_info])
}
