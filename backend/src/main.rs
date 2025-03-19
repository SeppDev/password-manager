#[macro_use]
extern crate rocket;



mod database;
mod api;

#[launch]
async fn rocket() -> _ {
    let db = database::create_connection().await.unwrap();

    rocket::build().manage(db).mount("/api", routes![api::signup, api::fetch_users])
}

// const PASSWORD: &str = "password";

// fn main() {


//     let h = hash(PASSWORD, DEFAULT_COST).unwrap();
//     let v = verify("password2", &h).unwrap();
//     println!("{h}\n{v}");

// }