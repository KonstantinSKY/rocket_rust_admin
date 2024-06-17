pub mod settings;
pub mod routes;
pub mod models;
pub mod database;
pub mod services;

use crate::project::AppSettings;

pub fn get_app_settings() -> AppSettings {
    AppSettings { 
        name : settings::NAME, 
        verbose_name: settings::VERBOSE_NAME,
        routes: settings::routes(),
  } 
}