#[macro_use]
extern crate rocket;

use database::Database;

mod api;
mod cors;
mod database;
pub mod jwt;
pub mod server;
mod tests;

use dotenv::dotenv;
use server::build;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");

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

    build(db).await
}
