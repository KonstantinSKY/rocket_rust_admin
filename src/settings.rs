// use rocket::routes;
use rocket::Route;

// // use crate::routes::hello::hello;
// use crate::routes::hi_json;
use crate::routes::{hello, hi_json};

// Routers settings
// pub const ROUTE_HANDLERS: &[fn() -> Route] = &[
//     hello::hello,
//     hi_json::hi_json,
// ];

// routes 
pub fn get_routes() -> Vec<Route> {
    routes![
        hi_json::hi_json,
        hello::hello,
        ]
}