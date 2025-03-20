use database::Database;

#[macro_use]
extern crate rocket;

mod api;
mod database;
mod tests;

#[launch]
async fn rocket() -> _ {
    let db = Database::open("database.db").await.unwrap();
    db.init_connection().await;

    rocket::build()
        .manage(db)
        .mount("/api", routes![api::signup, api::login, api::fetch_users])
}
