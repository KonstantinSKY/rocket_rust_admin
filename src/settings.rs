use rocket::routes;

use crate::routes::{hello, hi_json};

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        hello::hello,
        hi_json::hi_json
    ]
}