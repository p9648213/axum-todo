use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
    Json, RequestExt,
};
use serde::Deserialize;
use validator::Validate;

use crate::utilities::app_error::AppError;

#[derive(Debug, Validate, Deserialize)]
pub struct ValidatedCreatedTask {
    #[validate(length(min = 1, max = 1, message = "Priority must be a single character"))]
    pub priority: Option<String>,
    #[validate(required(message = "missing task title"))]
    pub title: Option<String>,
    pub description: Option<String>,
}

#[async_trait]
impl<S> FromRequest<S> for ValidatedCreatedTask
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(task) = req
            .extract::<Json<ValidatedCreatedTask>, _>()
            .await
            .map_err(|error| {
                eprintln!("Error extracting new task: {:?}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
            })?;

        if let Err(errors) = task.validate() {
            let errors = errors.field_errors();
            for (_, error) in errors {
                for error_message in error {
                    return Err(AppError::new(
                        StatusCode::BAD_REQUEST,
                        format!("{}", error_message.to_string()),
                    ));
                }
            }
        }

        Ok(task)
    }
}
