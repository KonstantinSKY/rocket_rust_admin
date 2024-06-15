use std::env;

use dotenv::dotenv;
use rocket::{figment::Figment, Config, Route};
// use crate::routes::{hello, hi_json, users};

//  === Routes 
fn get_routes() -> Vec<Route> {
    let mut routes = Vec::<Route>::new();
    routes
        .extend(crate::auth::settings::routes());
    
    routes
    
}

pub struct Settings {
    pub secret_key: String,
    pub figment: Figment,
    pub routes: Vec<Route>,
}

impl Settings {
    pub fn new () -> Self {
        dotenv().ok(); // Load environment variables
        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let figment = Config::figment()   
                                .merge(("secret_key", secret_key.clone()));
        
        // let apps = Apps::new();

        Self {
            secret_key,
            figment,
            routes: get_routes(),
        }        
    }
    
    
}


