#[macro_use]
extern crate rocket;

mod api;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![api::test::hello])
}
