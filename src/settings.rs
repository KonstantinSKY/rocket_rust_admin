use std::env;

use dotenv::dotenv;
use rocket::{figment::Figment, Config, Route};
// use crate::routes::{hello, hi_json, users};

 use crate::auth::settings as auth;
   





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
        
        Self {
            secret_key,
            figment,
            routes: get_routes(),
        }        
    }
    
    
}

//  === Routes 
fn get_routes() -> Vec<Route> {
    let mut routes = Vec::<Route>::new();

    routes.append(&mut settings::get_routes());
    routes

    // routes![
    //     auth::settings::get_routes(),
    //     // hi_json::hi_json,
    //     // hello::hello,
    //     // users::get_all_users,
    //     ]
}

