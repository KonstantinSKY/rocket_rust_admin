use rocket::serde::json::{Value, json};
#[get("/")]
pub fn hi_json() -> Value {
    json!("{key: Hello, world, JSON!}")
}