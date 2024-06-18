// auth  app settings

use rocket::Route;
use super::routes as R;

pub const NAME: &str = "auth";
pub const VERBOSE_NAME: &str = "Authentication";


// Add App Routes here
//  === Routes 
pub fn routes() -> Vec<Route> {
    routes![
        R::hi_json::hi_json,
        R::hello::hello,
        R::users::get_all_users,
        R::users::add_user,
        R::users::delete_user,
        R::groups::get_all_groups,
        R::groups::add_group,
        R::groups::delete_group,
        R::user_group::get_all,
        R::user_group::add_one,
        ]
}
