use std::collections::HashMap;

use diesel::PgConnection;

use crate::{
    dto::course::{CourseResponse, CreateCourseRequest, UpdateCourseRequest},
    error::app_error::AppError,
    models::course::{NewCourse, UpdateCourse},
    repository::course_repository::CourseRepository,
};

pub struct CourseService;

impl CourseService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateCourseRequest,
    ) -> Result<CourseResponse, AppError> {
        let query_params = HashMap::from([
            (
                "master_course_id".to_string(),
                request.master_course_id.to_string(),
            ),
            ("semester_id".to_string(), request.semester_id.to_string()),
            ("major_id".to_string(), request.major_id.to_string()),
            (
                "section_number".to_string(),
                request.section_number.to_string(),
            ),
        ]);

        if !CourseRepository::find_all(conn, &query_params)
            .unwrap_or_else(|_| Vec::new())
            .is_empty()
        {
            return Err(AppError::CourseAlreadyExists);
        }

        let new_course = NewCourse {
            master_course_id: request.master_course_id,
            semester_id: request.semester_id,
            major_id: request.major_id,

            description: request.description,

            course_category: request.course_category,

            language: request.language,

            section_number: request.section_number,
            grade: request.grade,
            capacity: request.capacity,
        };

        CourseRepository::create(conn, &new_course).map_err(|_| AppError::DatabaseError)?;

        let course = CourseRepository::find_all(conn, &query_params)
            .map_err(|_| AppError::DatabaseError)?
            .into_iter()
            .next()
            .unwrap_or_else(|| unreachable!());

        Ok(course.into())
    }

    pub fn get_by_id(conn: &mut PgConnection, course_id: i64) -> Result<CourseResponse, AppError> {
        let course =
            CourseRepository::find_by_id(conn, course_id).map_err(|_| AppError::CourseNotFound)?;

        Ok(course.into())
    }

    pub fn get_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> Result<Vec<CourseResponse>, AppError> {
        let courses =
            CourseRepository::find_all(conn, params).map_err(|_| AppError::DatabaseError)?;

        Ok(courses.into_iter().map(Into::into).collect())
    }

    pub fn update(
        conn: &mut PgConnection,
        course_id: i64,
        request: UpdateCourseRequest,
    ) -> Result<CourseResponse, AppError> {
        CourseRepository::find_by_id(conn, course_id).map_err(|_| AppError::CourseNotFound)?;

        let update_course = UpdateCourse {
            description: request.description,

            course_category: request.course_category,

            language: request.language,

            section_number: request.section_number,
            grade: request.grade,
            capacity: request.capacity,
        };

        CourseRepository::update(conn, course_id, &update_course)
            .map_err(|_| AppError::DatabaseError)?;

        let course =
            CourseRepository::find_by_id(conn, course_id).map_err(|_| AppError::DatabaseError)?;

        Ok(course.into())
    }

    pub fn delete(conn: &mut PgConnection, course_id: i64) -> Result<(), AppError> {
        CourseRepository::find_by_id(conn, course_id).map_err(|_| AppError::CourseNotFound)?;

        CourseRepository::delete(conn, course_id).map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}
