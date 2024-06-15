use rocket::serde::json::Json;
use rocket::{get, State};
use sea_orm::{EntityTrait, DatabaseConnection};
use super::super::models::user::{Entity as User, Model as UserModel};


#[get("/auth/users")]
pub async fn get_all_users(db: &State<DatabaseConnection>) -> Json<Vec<UserModel>> {
    Json(User::find().all(db.inner()).await.unwrap_or_default())
}
