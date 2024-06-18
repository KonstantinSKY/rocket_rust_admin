use rocket::State;
use sea_orm::{DatabaseConnection, DbErr};
use sea_orm::EntityTrait; // Ensure this is correctly imported


pub async fn select_all<E: EntityTrait>(db: &State<DatabaseConnection>) -> Result<Vec<E::Model>, DbErr> {
    
    let result = E::find()  // Use the find method on the entity
        .all(db.inner())  // Fetch all records
        .await;  // Since it's an async operation, wait for it

    match result {
        Ok(models) => Ok(models), // If successful, return the models
        Err(err) => Err(err) // If there is an error, return the error
    }
}