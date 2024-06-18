use rocket::{http::Status, serde::json::Json};
use sea_orm::DbErr;
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

// Generic function to handle the result and convert to JSON response
pub fn handle_selection_result_by_response_struct<T, R: From<T>>(result: Result<Vec<T>, DbErr>) -> Result<Json<Vec<R>>, Status>  {
    match result {
        Ok(models) => {
            let responses: Vec<R> = models.into_iter().map(R::from).collect();
            Ok(Json(responses))
        }
        Err(_) => {
            // Log the error internally if needed
            // error!("Failed to fetch data: {}", err);
            Err(Status::InternalServerError)
        }
    }
}


// Generic function to handle the result and convert to JSON response
pub fn handle_selection_result<T>(result: Result<Vec<T>, DbErr>) -> Result<Json<Vec<T>>, Status>  {
    match result {
        Ok(models) => {
            Ok(Json(models))
        }
        Err(err) => {
            // Log the error internally if needed
            // error!("Failed to fetch data: {}", err);
            Err(Status::InternalServerError)
        }
    }
}

// Generic function to handle the result and convert to JSON response
pub fn handle_insertion_result<T>(result: Result<T, DbErr>) -> Result<Json<T>, Status>  {
    match result {
        Ok(inserted_model) => {
            Ok(Json(inserted_model))
        }
        Err(err) => {
            // Log the error internally if needed
            // error!("Failed to fetch data: {}", err);
            Err(Status::InternalServerError)
        }
    }
}

pub fn handle_insertion_result_by_response_struct<T, R: From<T>>(result: Result<T, DbErr>) -> Result<Json<R>, Status>  {
    match result {
        Ok(inserted_model) => {
            // let response: Vec<R> = models.into_iter().map(R::from).collect();
            let response = R::from(inserted_model);
            Ok(Json(response))
        }
        Err(_) => {
            // Log the error internally if needed
            // error!("Failed to fetch data: {}", err);
            Err(Status::InternalServerError)
        }
    }
}
