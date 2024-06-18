use std::result;

use rocket::serde::json::Json;
use rocket::{get, State};
use sea_orm::{entity::*, DatabaseConnection};
use serde::{Deserialize, Serialize};
use rocket::http::Status;
use super::super::models::user;
use chrono::{NaiveDateTime, Utc};
use crate::db::{select, insert, delete};
use validator::{Validate, ValidationError, ValidationErrors};
use crate::project::responses;

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
#[derive(Serialize)]
struct ValidationErrorResponse {
    field: String,
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    errors: Vec<ValidationErrorResponse>,
}

// Define the NewUser struct
#[derive(Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 1, max = 15))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    #[validate(length(min = 1, max = 20))]
    pub first_name: Option<String>,
    #[validate(length(min = 1, max = 20))]
    pub last_name: Option<String>,
}

// Handler to add a new user
#[post("/auth/users", data = "<new_user>")]
pub async fn add_user(db: &State<DatabaseConnection>, new_user: Json<NewUser>) -> Result<Json<UserResponse>, rocket::http::Status> {
    
    if let Err(validation_errors) = new_user.validate() {
        let errors = validation_errors_to_response(validation_errors);
        return Err(Status::BadRequest);
    }

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


// Handler to delete a user
#[delete("/auth/users/<user_id>")]
pub async fn delete_user(db: &State<DatabaseConnection>, user_id: i32) -> Result<Status, Status> {
    let result = user::Entity::delete_by_id(user_id).exec(db.inner()).await;    // Correct
    responses::handle_deletion_result(result)
}



// Hash password function
fn hash_password(password: String) -> String {
    // Implement your hashing logic here, for example using bcrypt
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}

fn validation_errors_to_response(errors: ValidationErrors) -> ErrorResponse {
    let mut error_responses = Vec::new();

    for (field, errors) in errors.field_errors() {
        for error in errors {
            let message = error.message.clone().unwrap_or_else(|| "Invalid value".into());
            error_responses.push(ValidationErrorResponse {
                field: field.to_string(),
                message: message.to_string(),
            });
        }
    }

    ErrorResponse { errors: error_responses }
}