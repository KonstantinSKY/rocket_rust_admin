
#[macro_use] extern crate rocket;
use api::open_api;

mod routes;
mod api;
mod models;
mod services;
mod settings;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", settings::get_routes())
        .mount("/doc", open_api::get_open_api_routes())
        .mount("/doc/swagger/", open_api::get_swagger_routes())
        .mount("/doc/rapidoc/", open_api::get_rapidoc_routes())
        .launch()
        .await;
}
