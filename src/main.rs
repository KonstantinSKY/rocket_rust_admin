#[macro_use] extern crate rocket;

use rocket::serde::json::{Value, json};

#[get("/")]
fn hello() -> &'static str {
    "Hello, world, Hey!"
}

#[get("/")]
fn hi_json() -> Value {
    json!("Hello, world, Hey!")
}

// #[rocket::main]
// async fn main() {
//     let _ = rocket::build()
//         .mount("/", routes![hello])
//         .launch()
//         .await;
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/hi_json", routes![hi_json])
}