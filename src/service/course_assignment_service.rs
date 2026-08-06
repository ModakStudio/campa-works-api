use diesel::prelude::*;
use std::collections::HashMap;

use crate::{
    dto::course_assignment::{CourseAssignmentResponse, CreateCourseAssignmentRequest},
    error::app_error::AppError,
    models::course_assignment::NewCourseAssignment,
    repository::{
        course_assignment_repository::CourseAssignmentRepository,
        course_repository::CourseRepository, professor_repository::ProfessorRepository,
    },
};

pub struct CourseAssignmentService;

impl CourseAssignmentService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateCourseAssignmentRequest,
    ) -> Result<CourseAssignmentResponse, AppError> {
        CourseRepository::find_by_id(conn, request.course_id)
            .map_err(|_| AppError::CourseNotFound)?;

        ProfessorRepository::find_by_id(conn, request.professor_id)
            .map_err(|_| AppError::ProfessorNotFound)?;

        let query_params = HashMap::from([
            ("course_id".to_string(), request.course_id.to_string()),
            (
                "professor_id".to_string(),
                request.professor_id.to_string(),
            ),
        ]);

        if !CourseAssignmentRepository::find_all(conn, &query_params)
            .unwrap_or_else(|_| Vec::new())
            .is_empty()
        {
            return Err(AppError::CourseAssignmentAlreadyExists);
        }

        let new_course_assignment = NewCourseAssignment {
            course_id: request.course_id,
            professor_id: request.professor_id,
        };

        CourseAssignmentRepository::create(conn, &new_course_assignment)
            .map_err(|_| AppError::DatabaseError)?;

        let course_assignment =
            CourseAssignmentRepository::find_all(conn, &query_params)
                .map_err(|_| AppError::DatabaseError)?
                .into_iter()
                .next()
                .unwrap_or_else(|| unreachable!());

        Ok(course_assignment.into())
    }

    pub fn get_all(conn: &mut PgConnection, params: &HashMap<String, String>) -> Result<Vec<CourseAssignmentResponse>, AppError> {
        let course_assignments =
            CourseAssignmentRepository::find_all(conn, params).map_err(|_| AppError::DatabaseError)?;

        Ok(course_assignments.into_iter().map(Into::into).collect())
    }

    pub fn get_by_id(
        conn: &mut PgConnection,
        id: i64,
    ) -> Result<CourseAssignmentResponse, AppError> {
        let course_assignment = CourseAssignmentRepository::find_by_id(conn, id)
            .map_err(|_| AppError::CourseAssignmentNotFound)?;

        Ok(course_assignment.into())
    }

    pub fn delete(conn: &mut PgConnection, id: i64) -> Result<(), AppError> {
        CourseAssignmentRepository::find_by_id(conn, id)
            .map_err(|_| AppError::CourseAssignmentNotFound)?;

        CourseAssignmentRepository::delete(conn, id).map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}
