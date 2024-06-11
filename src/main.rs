
#[macro_use] extern crate rocket;
use rocket_okapi::{
    openapi_get_routes, rapidoc::{
        make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig
    }, 
    settings::UrlObject, swagger_ui::{
        make_swagger_ui, SwaggerUIConfig}
    };
    use crate::routes::{hello::hello, hi_json::hi_json};
// use crate::routes::hello::hello;
use crate::routes::hello::okapi_add_operation_for_hello_;

use settings::get_routes;

// use crate::routes::hi_json::hi_json;
use crate::routes::hi_json::okapi_add_operation_for_hi_json_;

mod routes;
mod api;
mod models;
mod services;
mod settings;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        // .mount("/", routes!(crate::routes::hi_json::hi_json))
        .mount("/", get_routes())
        .mount("/doc", openapi_get_routes![hi_json, hello])
        .mount(
            "/doc/swagger/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/doc/rapidoc/",
            api::open_api::rapidoc_routes()
            // make_rapidoc(&RapiDocConfig {
            //     general: GeneralConfig {
            //         spec_urls: vec![UrlObject::new("General", "../openapi.json")],
            //         ..Default::default()
            //     },
            //     hide_show: HideShowConfig {
            //         allow_spec_url_load: false,
            //         allow_spec_file_load: false,
            //         ..Default::default()
            //     },
            //     ..Default::default()
            // }),
            

        )
        .launch()
        .await;
}
