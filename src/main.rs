#[macro_use] extern crate rocket;

use routes::{hello, hi_json};

mod routes;
mod api;
mod models;
mod services;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello::hello])
        .mount("/hi_json", routes![hi_json::hi_json])
}