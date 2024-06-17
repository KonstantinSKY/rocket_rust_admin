use rocket::State;
use sea_orm::{entity::*, DatabaseConnection};
use super::super::models::{group::{
    ActiveModel as GroupActiveModel, Entity as Group, Model as GroupModel
}, user::Model};


pub async fn select_all_groups(db: &State<DatabaseConnection>) -> Vec<GroupModel>{
    Group::find().all(db.inner()).await.unwrap_or_default()
}

