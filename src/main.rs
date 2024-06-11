#[macro_use] extern crate rocket;

use routes::{hello, hi_json};

mod routes;
mod api;
mod models;
mod services;
mod settings;

// #[launch]
// async fn rocket() -> _ {
//     rocket::build()
//         .mount("/", routes![hello::hello])
//         .mount("/hi_json", routes![hi_json::hi_json])
// }


#[rocket::main]
async fn main() {
    let _ = rocket::build()
    .mount("/", settings::get_routes())
        .launch()
        .await;
}
