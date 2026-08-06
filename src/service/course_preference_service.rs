use diesel::prelude::*;
use std::collections::HashMap;

use crate::{
    dto::course_preference::{
        CoursePreferenceResponse, CreateCoursePreferenceRequest, UpdateCoursePreferenceRequest,
    },
    error::app_error::AppError,
    models::course_preference::{NewCoursePreference, UpdateCoursePreference},
    repository::{
        course_preference_repository::CoursePreferenceRepository,
        master_course_repository::MasterCourseRepository,
        professor_repository::ProfessorRepository, semester_repository::SemesterRepository,
    },
};

pub struct CoursePreferenceService;

impl CoursePreferenceService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateCoursePreferenceRequest,
    ) -> Result<CoursePreferenceResponse, AppError> {
        SemesterRepository::find_by_id(conn, request.semester_id)
            .map_err(|_| AppError::SemesterNotFound)?;

        ProfessorRepository::find_by_id(conn, request.professor_id)
            .map_err(|_| AppError::ProfessorNotFound)?;

        MasterCourseRepository::find_by_id(conn, request.master_course_id)
            .map_err(|_| AppError::MasterCourseNotFound)?;

        let query_params = HashMap::from([
            ("semester_id".to_string(), request.semester_id.to_string()),
            ("professor_id".to_string(), request.professor_id.to_string()),
            (
                "master_course_id".to_string(),
                request.master_course_id.to_string(),
            ),
        ]);

        if CoursePreferenceRepository::find_all(conn, &query_params)
            .unwrap_or_else(|_| Vec::new())
            .is_empty()
        {
            return Err(AppError::CoursePreferenceAlreadyExists);
        }

        let new_course_preference = NewCoursePreference {
            semester_id: request.semester_id,
            professor_id: request.professor_id,
            master_course_id: request.master_course_id,
            priority: request.priority,
        };

        CoursePreferenceRepository::create(conn, &new_course_preference)
            .map_err(|_| AppError::DatabaseError)?;

        let course_preference = CoursePreferenceRepository::find_all(conn, &query_params)
            .map_err(|_| AppError::DatabaseError)?
            .into_iter()
            .next()
            .unwrap_or_else(|| unreachable!());

        Ok(course_preference.into())
    }

    pub fn get_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> Result<Vec<CoursePreferenceResponse>, AppError> {
        let course_preferences = CoursePreferenceRepository::find_all(conn, params)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(course_preferences.into_iter().map(Into::into).collect())
    }

    pub fn get_by_id(
        conn: &mut PgConnection,
        id: i64,
    ) -> Result<CoursePreferenceResponse, AppError> {
        let course_preference = CoursePreferenceRepository::find_by_id(conn, id)
            .map_err(|_| AppError::CoursePreferenceNotFound)?;

        Ok(course_preference.into())
    }

    pub fn update(
        conn: &mut PgConnection,
        course_preference_id: i64,
        request: UpdateCoursePreferenceRequest,
    ) -> Result<CoursePreferenceResponse, AppError> {
        CoursePreferenceRepository::find_by_id(conn, course_preference_id)
            .map_err(|_| AppError::CoursePreferenceNotFound)?;

        let updated_course_preference = UpdateCoursePreference {
            priority: request.priority,
        };

        CoursePreferenceRepository::update(conn, course_preference_id, &updated_course_preference)
            .map_err(|_| AppError::DatabaseError)?;

        let course_preference = CoursePreferenceRepository::find_by_id(conn, course_preference_id)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(course_preference.into())
    }

    pub fn delete(conn: &mut PgConnection, id: i64) -> Result<(), AppError> {
        CoursePreferenceRepository::find_by_id(conn, id)
            .map_err(|_| AppError::CoursePreferenceNotFound)?;

        CoursePreferenceRepository::delete(conn, id).map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}
