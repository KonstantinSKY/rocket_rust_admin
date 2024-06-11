#[macro_use] extern crate rocket;

use routes::{hello, hi_json};

mod routes;
mod api;
mod models;
mod services;


// #[launch]
// async fn rocket() -> _ {
//     rocket::build()
//         .mount("/", routes![hello::hello])
//         .mount("/hi_json", routes![hi_json::hi_json])
// }


#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![hello::hello])
        .mount("/hi_json", routes![hi_json::hi_json])
        .launch()
        .await;
}
