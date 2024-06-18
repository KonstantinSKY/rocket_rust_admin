pub mod errors;
pub mod responses;
pub mod validators;


use std::env;
use dotenv::dotenv;

use rocket::{figment::Figment, Config, Route};
use crate::settings as project_setting;
// use rocket_okapi::settings;

pub struct AppSettings {
    pub name: &'static str,
    pub verbose_name: &'static str,
    pub routes: Vec<Route>,
}

pub struct ProjectSettings {
    pub secret_key: String,  
    pub figment: Figment,
    pub apps_settings: Vec<AppSettings>,
    pub routes: Vec<Route>,
}

impl ProjectSettings {
    pub fn new () -> Self {
        dotenv().ok();      // Load environment variables

        //secret key
        let secret_key = env::var(project_setting::SECRET_KEY_NAME).expect("SECRET KEY must be set");
        let figment = Config::figment()   
                                .merge(("secret_key", secret_key.clone()));
        
        let apps_settings = project_setting::app_setting_collection();
        let routes = get_routes(&apps_settings); 
        
        Self {
            secret_key,
            figment,
            apps_settings,
            routes,
        }        
    }


}

fn get_routes(app_settings: &Vec<AppSettings>) -> Vec<Route> {
    let mut routes = Vec::<Route>::new();
    for app_settings in app_settings{
        routes.extend(app_settings.routes.clone());
    }
    routes
}
