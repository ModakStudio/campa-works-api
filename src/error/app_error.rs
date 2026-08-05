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
    #[error("Professor not found")]
    ProfessorNotFound,
    #[error("Professor already exists")]
    ProfessorAlreadyExists,
    #[error("Semester not found")]
    SemesterNotFound,
    #[error("Semester already exists")]
    SemesterAlreadyExists,
    #[error("Major not found")]
    MajorNotFound,
    #[error("Major already exists")]
    MajorAlreadyExists,
    #[error("Master course not found")]
    MasterCourseNotFound,
    #[error("Master course already exists")]
    MasterCourseAlreadyExists,
    #[error("Course not found")]
    CourseNotFound,
    #[error("Course already exists")]
    CourseAlreadyExists,
    #[error("Course pool already exists")]
    CoursePoolAlreadyExists,
    #[error("Course pool not found")]
    CoursePoolNotFound,
    #[error("Course preference already exists")]
    CoursePreferenceAlreadyExists,
    #[error("Course preference not found")]
    CoursePreferenceNotFound,
    #[error("Course assignment already exists")]
    CourseAssignmentAlreadyExists,
    #[error("Course assignment not found")]
    CourseAssignmentNotFound,
    #[error("Professor credit already exists")]
    ProfessorCreditAlreadyExists,
    #[error("Professor credit not found")]
    ProfessorCreditNotFound,
    #[error("Course preference bookmark already exists")]
    CoursePreferenceBookmarkAlreadyExists,
    #[error("Course preference bookmark not found")]
    CoursePreferenceBookmarkNotFound,
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
            AppError::ProfessorNotFound => StatusCode::NOT_FOUND,
            AppError::ProfessorAlreadyExists => StatusCode::CONFLICT,
            AppError::SemesterNotFound => StatusCode::NOT_FOUND,
            AppError::SemesterAlreadyExists => StatusCode::CONFLICT,
            AppError::MajorNotFound => StatusCode::NOT_FOUND,
            AppError::MajorAlreadyExists => StatusCode::CONFLICT,
            AppError::MasterCourseNotFound => StatusCode::NOT_FOUND,
            AppError::MasterCourseAlreadyExists => StatusCode::CONFLICT,
            AppError::CourseNotFound => StatusCode::NOT_FOUND,
            AppError::CourseAlreadyExists => StatusCode::CONFLICT,
            AppError::CoursePoolAlreadyExists => StatusCode::CONFLICT,
            AppError::CoursePoolNotFound => StatusCode::NOT_FOUND,
            AppError::CoursePreferenceAlreadyExists => StatusCode::CONFLICT,
            AppError::CoursePreferenceNotFound => StatusCode::NOT_FOUND,
            AppError::CourseAssignmentAlreadyExists => StatusCode::CONFLICT,
            AppError::CourseAssignmentNotFound => StatusCode::NOT_FOUND,
            AppError::ProfessorCreditAlreadyExists => StatusCode::CONFLICT,
            AppError::ProfessorCreditNotFound => StatusCode::NOT_FOUND,
            AppError::CoursePreferenceBookmarkAlreadyExists => StatusCode::CONFLICT,
            AppError::CoursePreferenceBookmarkNotFound => StatusCode::NOT_FOUND,
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
