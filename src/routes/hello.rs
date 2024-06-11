// use rocket_okapi::openapi;

// #[openapi]
#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, world!"
}