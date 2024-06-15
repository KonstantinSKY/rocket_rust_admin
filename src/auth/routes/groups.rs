use rocket::serde::json::Json;
use rocket::{get, State};
use sea_orm::{EntityTrait, DatabaseConnection};
use super::super::models::group::{Entity as Group, Model as GroupModel};


#[get("/auth/groups")]
pub async fn get_all_groups(db: &State<DatabaseConnection>) -> Json<Vec<GroupModel>> {
    Json(Group::find().all(db.inner()).await.unwrap_or_default())
}
