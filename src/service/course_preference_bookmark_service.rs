use diesel::prelude::*;
use std::collections::HashMap;

use crate::{
    dto::course_preference_bookmark::{
        CoursePreferenceBookmarkResponse, CreateCoursePreferenceBookmarkRequest,
    },
    error::app_error::AppError,
    models::course_preference_bookmark::NewCoursePreferenceBookmark,
    repository::{
        course_preference_bookmark_repository::CoursePreferenceBookmarkRepository,
        master_course_repository::MasterCourseRepository,
        professor_repository::ProfessorRepository,
    },
};

pub struct CoursePreferenceBookmarkService;

impl CoursePreferenceBookmarkService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateCoursePreferenceBookmarkRequest,
    ) -> Result<CoursePreferenceBookmarkResponse, AppError> {
        ProfessorRepository::find_by_id(conn, request.professor_id)
            .map_err(|_| AppError::ProfessorNotFound)?;

        MasterCourseRepository::find_by_id(conn, request.master_course_id)
            .map_err(|_| AppError::MasterCourseNotFound)?;

        let query_params = HashMap::from([
            ("professor_id".to_string(), request.professor_id.to_string()),
            (
                "master_course_id".to_string(),
                request.master_course_id.to_string(),
            ),
        ]);

        if !CoursePreferenceBookmarkRepository::find_all(conn, &query_params)
            .unwrap_or_else(|_| Vec::new())
            .is_empty()
        {
            return Err(AppError::CoursePreferenceBookmarkAlreadyExists);
        }

        let new_course_preference_bookmark = NewCoursePreferenceBookmark {
            professor_id: request.professor_id,
            master_course_id: request.master_course_id,
        };

        CoursePreferenceBookmarkRepository::create(conn, &new_course_preference_bookmark)
            .map_err(|_| AppError::DatabaseError)?;

        let course_preference_bookmark =
            CoursePreferenceBookmarkRepository::find_all(conn, &query_params)
                .map_err(|_| AppError::DatabaseError)?
                .into_iter()
                .next()
                .unwrap_or_else(|| unreachable!());

        Ok(course_preference_bookmark.into())
    }

    pub fn get_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> Result<Vec<CoursePreferenceBookmarkResponse>, AppError> {
        let course_preference_bookmarks =
            CoursePreferenceBookmarkRepository::find_all(conn, params)
                .map_err(|_| AppError::DatabaseError)?;

        Ok(course_preference_bookmarks
            .into_iter()
            .map(Into::into)
            .collect())
    }

    pub fn get_by_id(
        conn: &mut PgConnection,
        course_preference_bookmark_id: i64,
    ) -> Result<CoursePreferenceBookmarkResponse, AppError> {
        let course_preference_bookmark =
            CoursePreferenceBookmarkRepository::find_by_id(conn, course_preference_bookmark_id)
                .map_err(|_| AppError::DatabaseError)?;

        Ok(course_preference_bookmark.into())
    }

    pub fn delete(
        conn: &mut PgConnection,
        course_preference_bookmark_id: i64,
    ) -> Result<(), AppError> {
        CoursePreferenceBookmarkRepository::find_by_id(conn, course_preference_bookmark_id)
            .map_err(|_| AppError::CoursePreferenceBookmarkNotFound)?;

        CoursePreferenceBookmarkRepository::delete(conn, course_preference_bookmark_id)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}
