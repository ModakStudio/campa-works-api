use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("User not found")]
    UserNotFound,

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Database error")]
    DatabaseError,

    #[error("Internal server error")]
    Internal,
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::UserNotFound => StatusCode::NOT_FOUND,

            AppError::EmailAlreadyExists => StatusCode::CONFLICT,

            AppError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,

            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (
            status,
            Json(ErrorResponse {
                success: false,
                message: self.to_string(),
            }),
        )
            .into_response()
    }
}
