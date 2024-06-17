use std::env;

use crate::project::AppSettings;

use dotenv::dotenv;

use rocket::{figment::Figment, Config, Route};

// === ADD NEW APP HERE !!!
// Collect all app setting to one collection
fn app_setting_collection() -> Vec<AppSettings>{
    vec![
        crate::auth::get_app_settings(),
        // crate::new_app::settings::get(), // Uncomment and edit as needed
    ]
}

// pub struct AppSettings {
//     pub name: &'static str,
//     pub verbose_name: &'static str,
//     pub routes: Vec<Route>,
// }

//  === Routes 
fn get_routes(app_settings: &Vec<AppSettings>) -> Vec<Route> {
    let mut routes = Vec::<Route>::new();
    for app_settings in app_settings{
        routes.extend(app_settings.routes.clone());
    }
    routes
}

pub struct ProjectSettings {
    pub secret_key: String,  
    pub figment: Figment,
    pub apps_settings: Vec<AppSettings>,
    pub routes: Vec<Route>,
}

impl ProjectSettings {
    pub fn new () -> Self {
        dotenv().ok(); // Load environment variables

        //secret key
        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let figment = Config::figment()   
                                .merge(("secret_key", secret_key.clone()));
        
        let apps_settings = app_setting_collection();
        let routes = get_routes(&apps_settings); 
        
        Self {
            secret_key,
            figment,
            apps_settings,
            routes,
        }        
    }
}


