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
        if MasterCourseRepository::find_by_course_code(conn, &request.course_code).is_ok() {
            return Err(AppError::MasterCourseAlreadyExists);
        }

        let new_course = NewMasterCourse {
            course_code: request.course_code,
            name: request.name,
            credit: request.credit,
            lecture: request.lecture,
            practice: request.practice,
            course_type: request.course_type,
            is_core: request.is_core,
        };

        let course = MasterCourseRepository::create(conn, &new_course)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(course.into())
    }

    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<MasterCourseResponse>, AppError> {
        let courses =
            MasterCourseRepository::find_all(conn).map_err(|_| AppError::DatabaseError)?;

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
            if let Ok(existing) = MasterCourseRepository::find_by_course_code(conn, code) {
                if existing.id != course_id {
                    return Err(AppError::MasterCourseAlreadyExists);
                }
            }
        }

        let update_course = UpdateMasterCourse {
            course_code: request.course_code,
            name: request.name,
            credit: request.credit,
            lecture: request.lecture,
            practice: request.practice,
            course_type: request.course_type,
            is_core: request.is_core,
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
