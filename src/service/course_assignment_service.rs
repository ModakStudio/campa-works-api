use diesel::prelude::*;

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

        if CourseAssignmentRepository::find_by_course_id(conn, request.course_id).is_ok() {
            return Err(AppError::CourseAssignmentAlreadyExists);
        }

        let new_course_assignment = NewCourseAssignment {
            course_id: request.course_id,
            professor_id: request.professor_id,
        };

        CourseAssignmentRepository::create(conn, &new_course_assignment)
            .map_err(|_| AppError::DatabaseError)?;

        let course_assignment =
            CourseAssignmentRepository::find_by_course_id(conn, request.course_id)
                .map_err(|_| AppError::DatabaseError)?;

        Ok(course_assignment.into())
    }

    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<CourseAssignmentResponse>, AppError> {
        let course_assignments =
            CourseAssignmentRepository::find_all(conn).map_err(|_| AppError::DatabaseError)?;

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
