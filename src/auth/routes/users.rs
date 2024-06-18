use rocket::serde::json::Json;
use rocket::{get, State};
use sea_orm::{entity::*, DatabaseConnection};
use serde::Deserialize;
use super::super::models::user::{Entity as User, Model as UserModel};
use rocket::http::Status;
use super::super::models::user;
use chrono::Utc;
use crate::db::{select, insert};
use bcrypt::{hash, DEFAULT_COST, BcryptError};

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


// Define the NewUser struct
#[derive(Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

// Handler to add a new user
#[post("/auth/users", data = "<new_user>")]
pub async fn add_user(db: &State<DatabaseConnection>, new_user: Json<NewUser>) -> Result<Json<user::Model>, rocket::http::Status> {
    
    let active_user = user::ActiveModel {
        name: Set(new_user.name.clone()),
        email: Set(new_user.email.clone()),
        password: Set(hash_password(new_user.password.clone())), // Assuming you have a function to hash passwords
        first_name: Set(new_user.first_name.clone()),
        last_name: Set(new_user.last_name.clone()),
        created_at: Set(Utc::now().naive_utc()),
        is_active: Set(true), // Default values for new users
        is_staff: Set(false),
        is_superuser: Set(false),
        last_login: Set(None),
        ..Default::default()
    };

    let insert_result = insert::insert::<user::Entity, _>(db, active_user).await;

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

// Hash password function
fn hash_password(password: String) -> String {
    // Implement your hashing logic here, for example using bcrypt
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}