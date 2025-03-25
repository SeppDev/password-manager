#[macro_use]
extern crate rocket;

use database::Database;

mod api;
mod database;
mod tests;

use dotenv::dotenv;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");

    let db = Database::open(&database_url).await.unwrap();
    db.init_connection().await;

    rocket::build()
        .manage(db)
        .mount("/api", routes![api::signup, api::login, api::fetch_users])
}
