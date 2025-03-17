
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
    alive: bool
}

#[get("/hello")]
pub fn hello() -> String {
    let user = User {
        name: "Jon Snow".to_string(),
        age: 21,
        alive: true,
    };
    serde_json::to_string(&user).unwrap()
}