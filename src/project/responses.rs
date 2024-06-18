use rocket::http::Status;
// === Universal Result handlers


pub fn handle_deletion_result(result: Result<sea_orm::DeleteResult, sea_orm::DbErr>) -> Result <Status, Status> {
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