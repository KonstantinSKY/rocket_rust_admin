#[macro_use] extern crate rocket;
use dotenv::dotenv;

mod db;
mod routes;
mod api;
mod models;
mod services;
mod settings;

#[rocket::main]
async fn main() {
    dotenv().ok();                                          // Load environment variables

    let _ = rocket::build()
        .attach(db::stage())                               // Attach your database stage
        .mount("/", settings::get_routes())                // Mount your application routes
        .mount("/doc", api::get_open_api_routes())         // Mount OpenAPI routes
        .mount("/doc/swagger/", api::get_swagger_routes()) // Mount Swagger UI routes
        .mount("/doc/rapidoc/", api::get_rapidoc_routes()) // Mount RapiDoc routes
        .launch()
        .await;
}
