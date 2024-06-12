use rocket::Route;
use rocket_okapi::{
    openapi_get_routes, rapidoc::{
        make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig
    }, 
    settings::UrlObject, swagger_ui::{
        make_swagger_ui, SwaggerUIConfig
    ,}
};
use crate::routes::{
    hello::hello, hello::okapi_add_operation_for_hello_,
    hi_json::hi_json, hi_json::okapi_add_operation_for_hi_json_,
};


pub fn get_open_api_routes() -> Vec<Route> {
    openapi_get_routes![hi_json, hello]
    }

pub fn get_swagger_routes() -> Vec<Route> {
    make_swagger_ui(&SwaggerUIConfig {
        url: "../openapi.json".to_owned(),
            ..Default::default()
        }).into()
    }

    // Function to create routes for RapiDoc
pub fn get_rapidoc_routes() -> Vec<Route> {
    make_rapidoc(&RapiDocConfig {
        general: GeneralConfig {
            spec_urls: vec![UrlObject::new("General", "../openapi.json")],
            ..Default::default()
        },
        hide_show: HideShowConfig {
            allow_spec_url_load: false,
            allow_spec_file_load: false,
            ..Default::default()
        },
        ..Default::default()
    }).into()
    
}
