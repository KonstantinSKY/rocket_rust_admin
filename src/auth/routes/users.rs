use rocket_okapi::openapi;
use rocket::serde::json::Json;
use rocket::{get, State};
use sea_orm::{EntityTrait, DatabaseConnection};
use super::super::models::user::{Entity as User, Model as UserModel};
// use sea_orm::EntityTrait;


// #[openapi]
#[get("/auth/users")]
pub async fn get_all_users(db: &State<DatabaseConnection>) -> Json<Vec<UserModel>> {
    let users: Vec<UserModel> = User::find().all(db.inner()).await.unwrap_or_else(|_| vec![]);
    Json(users)
}

// #[openapi]
// #[get("/users_openapi")]
// pub async fn get_all_users_openapi(db: &State<DatabaseConnection>) -> Json<Vec<UserModel>> {
//     let users: Vec<UserModel> = User::find().all(db.inner()).await.unwrap_or_else(|_| vec![]);
//     Json(users)
// }