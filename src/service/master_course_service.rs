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
        if MasterCourseRepository::find_by_key_value(
            conn,
            &request.course_code,
            &request.course_name,
            request.credit,
            request.lecture,
            request.practice,
        )
        .is_ok()
        {
            return Err(AppError::MasterCourseAlreadyExists);
        }

        let new_course = NewMasterCourse {
            course_code: request.course_code,
            course_name: request.course_name,
            course_en_name: request.course_en_name,

            credit: request.credit,
            lecture: request.lecture,
            practice: request.practice,

            course_type: request.course_type,

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
        let mut check_course = MasterCourseRepository::find_by_id(conn, course_id)
            .map_err(|_| AppError::MasterCourseNotFound)?;
        if let Some(course_code) = &request.course_code {
            check_course.course_code = course_code.clone();
        }
        if let Some(course_name) = &request.course_name {
            check_course.course_name = course_name.clone();
        }
        if let Some(credit) = request.credit {
            check_course.credit = credit;
        }
        if let Some(lecture) = request.lecture {
            check_course.lecture = lecture;
        }
        if let Some(practice) = request.practice {
            check_course.practice = practice;
        }
        if MasterCourseRepository::find_by_key_value(
            conn,
            &check_course.course_code,
            &check_course.course_name,
            check_course.credit,
            check_course.lecture,
            check_course.practice,
        )
        .is_ok()
        {
            return Err(AppError::MasterCourseAlreadyExists);
        }

        let update_course = UpdateMasterCourse {
            course_code: request.course_code,
            course_name: request.course_name,
            course_en_name: request.course_en_name,

            credit: request.credit,
            lecture: request.lecture,
            practice: request.practice,

            course_type: request.course_type,

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
