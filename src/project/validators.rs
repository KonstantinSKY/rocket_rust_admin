use serde::Serialize;

#[derive(Serialize)]
struct ValidationErrorResponse {
    field: String,
    message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    errors: Vec<ValidationErrorResponse>,
}

use validator::{Validate, ValidationError, ValidationErrors};

pub fn validation_errors_to_response(errors: ValidationErrors) -> ErrorResponse {
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