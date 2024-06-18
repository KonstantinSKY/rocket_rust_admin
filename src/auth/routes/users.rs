use rocket::serde::json::Json;
use rocket::{get, State};
use sea_orm::{entity::*, DatabaseConnection};
use serde::{Deserialize, Serialize};
use super::super::models::user::{Entity as User, Model as UserModel};
use rocket::http::Status;
use super::super::models::user;
use chrono::{DateTime, NaiveDateTime, Utc};
use crate::db::{select, insert};
use bcrypt::{hash, DEFAULT_COST, BcryptError};

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub last_login: Option<NaiveDateTime>,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub created_at: NaiveDateTime,
}
impl From<user::Model> for UserResponse {
    fn from(user: user::Model) -> Self {
        UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            last_login: user.last_login,
            is_active: user.is_active,
            is_staff: user.is_staff,
            is_superuser: user.is_superuser,
            created_at: user.created_at,
        }
    }
}


#[get("/auth/users")]
pub async fn get_all_users(db: &State<DatabaseConnection>) -> Result<Json<Vec<UserResponse>>, rocket::http::Status> {
    let result = select::select_all::<user::Entity>(db).await;
    
    match result {
        Ok(models) => {
            let user_responses: Vec<UserResponse> = models
                .into_iter()
                .map(UserResponse::from)
                .collect();
            Ok(Json(user_responses))
        },
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
pub async fn add_user(db: &State<DatabaseConnection>, new_user: Json<NewUser>) -> Result<Json<UserResponse>, rocket::http::Status> {
    
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
            let user_response = UserResponse::from(inserted_model);
            Ok(Json(user_response))
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