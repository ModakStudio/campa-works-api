use diesel::prelude::*;

use crate::{
    dto::course_pool::{CoursePoolResponse, CreateCoursePoolRequest},
    error::app_error::AppError,
    models::course_pool::NewCoursePool,
    repository::{
        course_pool_repository::CoursePoolRepository,
        master_course_repository::MasterCourseRepository,
        professor_repository::ProfessorRepository,
    },
};

pub struct CoursePoolService;

impl CoursePoolService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateCoursePoolRequest,
    ) -> Result<CoursePoolResponse, AppError> {
        ProfessorRepository::find_by_id(conn, request.professor_id)
            .map_err(|_| AppError::ProfessorNotFound)?;

        MasterCourseRepository::find_by_id(conn, request.master_course_id)
            .map_err(|_| AppError::MasterCourseNotFound)?;

        if CoursePoolRepository::find_by_professor_id_and_master_course_id(
            conn,
            request.professor_id,
            request.master_course_id,
        )
        .is_ok()
        {
            return Err(AppError::CoursePoolAlreadyExists);
        }

        let new_course_pool = NewCoursePool {
            professor_id: request.professor_id,
            master_course_id: request.master_course_id,
        };

        CoursePoolRepository::create(conn, &new_course_pool)
            .map_err(|_| AppError::DatabaseError)?;

        let course_pool = CoursePoolRepository::find_by_professor_id_and_master_course_id(
            conn,
            request.professor_id,
            request.master_course_id,
        )
        .map_err(|_| AppError::DatabaseError)?;

        Ok(course_pool.into())
    }

    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<CoursePoolResponse>, AppError> {
        let course_pools =
            CoursePoolRepository::find_all(conn).map_err(|_| AppError::DatabaseError)?;

        Ok(course_pools.into_iter().map(Into::into).collect())
    }

    pub fn get_by_id(conn: &mut PgConnection, id: i64) -> Result<CoursePoolResponse, AppError> {
        let course_pool =
            CoursePoolRepository::find_by_id(conn, id).map_err(|_| AppError::CoursePoolNotFound)?;

        Ok(course_pool.into())
    }

    pub fn delete(conn: &mut PgConnection, id: i64) -> Result<(), AppError> {
        CoursePoolRepository::find_by_id(conn, id).map_err(|_| AppError::CoursePoolNotFound)?;

        CoursePoolRepository::delete(conn, id).map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}
