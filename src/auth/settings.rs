// auth  app settings
use rocket::Route;
use super::routes as R;

// pub struct Settings {
//     pub routes: Vec<Route>,
// }

// impl Settings {
//     pub fn new () -> Self {

//         Self {
//             routes: get_routes(),
//         }
//     }
// }
//  === Routes 
pub fn routes() -> Vec<Route> {
    routes![
        R::hi_json::hi_json,
        R::hello::hello,
        R::users::get_all_users,
        ]
}

