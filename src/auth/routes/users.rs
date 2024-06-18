use rocket::serde::json::Json;
use rocket::{get, State};
use sea_orm::{EntityTrait, DatabaseConnection};
use super::super::models::user::{Entity as User, Model as UserModel};
use rocket::http::Status;
use super::super::models::user;
use crate::db::select;



#[get("/auth/users")]
pub async fn get_all_users(db: &State<DatabaseConnection>) -> Result<Json<Vec<user::Model>>, rocket::http::Status> {
    let result = select::select_all::<user::Entity>(db).await;
    
    match result {
        Ok(models) => Ok(Json(models)),
        Err(err) => {                                        // Log the error or handle it as needed
            //   error!("Failed to fetch groups: {}", err)          // Log the error internally
             Err(Status::InternalServerError)                       // Respond with only the HTTP status code
        }
    }
}