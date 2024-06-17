
use rocket::{figment::Figment, Config, Route};

pub struct AppSettings {
    pub name: &'static str,
    pub verbose_name: &'static str,
    pub routes: Vec<Route>,
}
