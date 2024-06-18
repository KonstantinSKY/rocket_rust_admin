use chrono::Utc;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};
use rocket::{get, post, State};
use sea_orm::{entity::*, DatabaseConnection};
use super::super::models::group;
use crate::db::{select, insert};

#[get("/auth/groups")]
pub async fn get_all_groups(db: &State<DatabaseConnection>) -> Result<Json<Vec<group::Model>>, rocket::http::Status> {
    let result = select::select_all::<group::Entity>(db).await;
    
    match result {
        Ok(models) => Ok(Json(models)),
        Err(err) => { // Log the error or handle it as needed
              // Log the error internally
            //   error!("Failed to fetch groups: {}", err);
              Err(Status::InternalServerError) // Respond with only the HTTP status code
        }
    }
}

// Post New group
#[derive(Deserialize)]
pub struct NewGroup {
    pub name: String,
    pub description: Option<String>,
}

// Handler to add a new group
#[post("/auth/groups", data = "<new_group>")]
pub async fn add_group(db: &State<DatabaseConnection>, new_group: Json<NewGroup>) -> Result<Json<group::Model>, rocket::http::Status> {
    
    let active_group = group::ActiveModel {
        name: Set(new_group.name.clone()),
        description: Set(new_group.description.clone()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let insert_result = insert::insert::<group::Entity, _>(db, active_group).await;

    match insert_result {
        Ok(inserted_model) => {
            Ok(Json(inserted_model))
        },
        Err(error) => {
            println!("Insert error: {:?}", error);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}

// Handler to delete a user
#[delete("/auth/groups/<group_id>")]
pub async fn delete_group(db: &State<DatabaseConnection>, group_id: i32) -> Result<Status, Status> {
    let result = group::Entity::delete_by_id(group_id).exec(db.inner()).await;    // Correct
    
    match result {
        Ok(result) => {
            if result.rows_affected > 0 {
                Ok(Status::NoContent) // Return 204 No Content if deletion was successful
            } else {
                Err(Status::NotFound) // Return 404 Not Found if no rows were affected
            }
        }
        Err(err) => {
            // println!("Delete error: {:?}", err);
            Err(Status::InternalServerError) // Return 500 Internal Server Error on failure
        }
    }
}

