use database::Database;

#[macro_use]
extern crate rocket;

mod tests;
mod database;
mod api;

#[launch]
async fn rocket() -> _ {
    let db = Database::new_memory().await.unwrap();
    db.init_connection().await;

    rocket::build().manage(db).mount("/api", routes![api::signup, api::login, api::fetch_users])
}