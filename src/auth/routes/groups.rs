use rocket::serde::{json::Json, Deserialize};
use rocket::{get, post, State};
use sea_orm::{entity::*, DatabaseConnection};
use super::super::models::group::{
    Entity as Group, 
    Model as GroupModel,
    ActiveModel as GroupActiveModel,
};


#[get("/auth/groups")]
pub async fn get_all_groups(db: &State<DatabaseConnection>) -> Json<Vec<GroupModel>> {
    Json(Group::find().all(db.inner()).await.unwrap_or_default())
}


#[derive(Deserialize)]
pub struct NewGroup {
    pub name: String,
}
// Handler to add a new group
#[post("/auth/groups", data = "<new_group>")]
pub async fn add_group(db: &State<DatabaseConnection>, new_group: Json<NewGroup>) -> Json<GroupModel> {
    let group = GroupActiveModel {
        name: Set(new_group.name.clone()),
        created_at: Set(chrono::Utc::now().naive_utc()),  // Assuming created_at is automatically set
        ..Default::default()
    };

    let insert_result = Group::insert(group)
        .exec(db.inner())
        .await
        .expect("Failed to insert new group");

    
     // Fetch the newly created group using the last inserted ID
     let new_group = Group::find_by_id(insert_result.last_insert_id)
    .one(db.inner())
    .await
    .expect("Failed to retrieve new group")
    .expect("New group not found");

    Json(new_group) 
    
}