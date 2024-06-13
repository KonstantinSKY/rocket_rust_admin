#[macro_use] extern crate rocket;

// mod db;
mod routes;
mod api;
mod models;
mod services;
mod settings;

#[rocket::main]
async fn main() {
    let sets = settings::Settings::new();                                        // Load environment variables

    let _ = rocket::custom(sets.figment)
        .attach(db::stage())                               // Attach database stage
        .mount("/", sets.routes)                              // Mount application routes
        .mount("/doc", api::get_open_api_routes())            // Mount OpenAPI routes
        .mount("/doc/swagger/", api::get_swagger_routes())    // Mount Swagger UI routes
        .mount("/doc/rapidoc/", api::get_rapidoc_routes())    // Mount RapiDoc routes
        .launch()
        .await;
}
