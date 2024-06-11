// use rocket::serde::json::Json;
// use rocket::Route;
// use rocket_okapi::openapi_get_routes;
// use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
// use rocket_okapi::rapidoc::{make_rapidoc, RapiDocConfig};
// // use crate::routes::handlers::ROUTE_HANDLERS;
// // use crate::routes::hello::hello;
// use crate::routes::hi_json::hi_json;
// use crate::routes::hi_json::okapi_add_operation_for_hi_json_;
// use rocket::serde::json::Json;
// use rocket_okapi::settings::OpenApiSettings;
// use rocket_okapi::gen::OpenApiGenerator;
// use rocket_okapi::swagger_ui::UrlObject;
// use crate::routes::hi_json::okapi_add_operation_for_hi_json_;
// pub fn get_open_api() -> Vec<Route> {
//     let mut routes: Vec<Route> = openapi_get_routes![ROUTE_HANDLERS];
    
//     routes.append(&mut make_swagger_ui(&get_swagger_config()));
//     routes.append(&mut make_rapidoc(&get_rapidoc_config()));
    
//     routes
//}
// #[get("/openapi.json")]
// pub fn openapi_json() -> Json<String> {
//     let settings = OpenApiSettings::default();
//     let mut gen = OpenApiGenerator::new(&settings);
//     let routes = openapi_get_routes![hi_json];
//     let spec = gen.merge(routes);
//     Json(spec.to_pretty_json().unwrap())
// }


// pub fn get_open_api() -> Vec<Route> {
//     let mut routes = openapi_get_routes![hi_json];
//     routes.append(&mut swagger_ui_routes());
//     // routes.append(&mut rapidoc_routes());
//     routes
// }

// pub fn swagger_ui_routes() -> Vec<Route> {
//     make_swagger_ui(&SwaggerUIConfig {
//         url: "../openapi.json".to_owned(),
//         ..Default::default()
//     }).into()
// }

use rocket::Route;
use rocket_okapi::{
    openapi_get_routes, rapidoc::{
        make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig
    }, 
    settings::UrlObject, swagger_ui::{
        make_swagger_ui, SwaggerUIConfig}
    };

// Function to create routes for RapiDoc
pub fn rapidoc_routes() -> Vec<Route> {
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
