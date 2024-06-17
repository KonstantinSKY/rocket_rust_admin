use rocket::serde::{json::Json, Deserialize};
use rocket::{get, post, State};
use sea_orm::{entity::*, DatabaseConnection};
use super::super::models::group::{
    Entity as Group, 
    Model as GroupModel,
    ActiveModel as GroupActiveModel,
};
use super::super::database::group as database;

#[get("/auth/groups")]
pub async fn get_all_groups(db: &State<DatabaseConnection>) -> Json<Vec<GroupModel>> {
    Json(database::select_all_groups(db).await)
    // Json(Group::find().all(db.inner()).await.unwrap_or_default())
}

// Post New group
#[derive(Deserialize)]
pub struct NewGroup {
    pub name: String,
    pub description: String,
}
// Handler to add a new group
#[post("/auth/groups", data = "<new_group>")]
pub async fn add_group(db: &State<DatabaseConnection>, new_group: Json<NewGroup>) -> Json<GroupModel> {
    let group = GroupActiveModel {
        name: Set(new_group.name.clone()),
        description: Set(Some(new_group.description.clone())),
        created_at: Set(chrono::Utc::now().naive_utc()),  // Assuming created_at is automatically set
        ..Default::default()
    };
    println!("New group: {:#?}", group);
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
    println!("New group: {:#?}", new_group);
    Json(new_group) 
    
}