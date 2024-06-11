// use rocket::serde::json::{json, Value};
use rocket_okapi::openapi;


/// First api Hello world for test
#[openapi]
#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, world!"
}