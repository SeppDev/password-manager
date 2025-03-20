#[macro_use]
extern crate rocket;

use database::Database;

mod api;
mod database;
mod tests;

#[launch]
async fn rocket() -> _ {
    let db = Database::new_memory().await.unwrap();
    db.init_connection().await;

    rocket::build()
        .manage(db)
        .mount("/api", routes![api::signup, api::fetch_users])
}
