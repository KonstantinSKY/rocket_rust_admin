
#[macro_use] extern crate rocket;

mod db;
mod routes;
mod api;
mod models;
mod services;
mod settings;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .attach(db::stage()) 
        .mount("/", settings::get_routes())
        .mount("/doc", api::get_open_api_routes())
        .mount("/doc/swagger/", api::get_swagger_routes())
        .mount("/doc/rapidoc/", api::get_rapidoc_routes())
        .launch()
        .await;
}
