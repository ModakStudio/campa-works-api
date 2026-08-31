use std::collections::HashMap;

use diesel::PgConnection;

use crate::{
    dto::master_course::{
        CreateMasterCourseRequest, MasterCourseResponse, UpdateMasterCourseRequest,
    },
    error::app_error::AppError,
    models::master_course::{NewMasterCourse, UpdateMasterCourse},
    repository::master_course_repository::MasterCourseRepository,
};

pub struct MasterCourseService;

impl MasterCourseService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateMasterCourseRequest,
    ) -> Result<MasterCourseResponse, AppError> {
        let query_params =
            HashMap::from([("course_code".to_string(), request.course_code.clone())]);
        if !MasterCourseRepository::find_all(conn, &query_params)
            .unwrap_or_else(|_| Vec::new())
            .is_empty()
        {
            return Err(AppError::MasterCourseAlreadyExists);
        }

        let new_course = NewMasterCourse {
            course_code: request.course_code,
            course_name: request.course_name,
            course_en_name: request.course_en_name,
            course_type: request.course_type,
            is_core: request.is_core,
            course_status: request.course_status,
        };

        let course = MasterCourseRepository::create(conn, &new_course)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(course.into())
    }

    pub fn get_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> Result<Vec<MasterCourseResponse>, AppError> {
        let courses =
            MasterCourseRepository::find_all(conn, params).map_err(|_| AppError::DatabaseError)?;

        Ok(courses.into_iter().map(Into::into).collect())
    }

    pub fn get_by_id(
        conn: &mut PgConnection,
        course_id: i64,
    ) -> Result<MasterCourseResponse, AppError> {
        let course = MasterCourseRepository::find_by_id(conn, course_id)
            .map_err(|_| AppError::MasterCourseNotFound)?;

        Ok(course.into())
    }

    pub fn update(
        conn: &mut PgConnection,
        course_id: i64,
        request: UpdateMasterCourseRequest,
    ) -> Result<MasterCourseResponse, AppError> {
        MasterCourseRepository::find_by_id(conn, course_id)
            .map_err(|_| AppError::MasterCourseNotFound)?;

        if let Some(ref code) = request.course_code {
            let query_params = HashMap::from([("course_code".to_string(), code.clone())]);
            if !MasterCourseRepository::find_all(conn, &query_params)
                .unwrap_or_else(|_| Vec::new())
                .is_empty()
            {
                return Err(AppError::MasterCourseAlreadyExists);
            }
        }

        let update_course = UpdateMasterCourse {
            course_code: request.course_code,
            course_name: request.course_name,
            course_en_name: request.course_en_name,
            course_type: request.course_type,
            is_core: request.is_core,
            course_status: request.course_status,
        };

        let updated = MasterCourseRepository::update(conn, course_id, &update_course)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(updated.into())
    }

    pub fn delete(conn: &mut PgConnection, course_id: i64) -> Result<(), AppError> {
        MasterCourseRepository::find_by_id(conn, course_id)
            .map_err(|_| AppError::MasterCourseNotFound)?;

        MasterCourseRepository::delete(conn, course_id).map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}
