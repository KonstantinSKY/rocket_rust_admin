use rocket::Route;


use crate::routes::{hello, hi_json};


//  === Routes 
pub fn get_routes() -> Vec<Route> {
    routes![
        hi_json::hi_json,
        hello::hello,
        ]
}