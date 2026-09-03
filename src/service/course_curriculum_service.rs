use std::collections::HashMap;

use diesel::PgConnection;

use crate::{
    dto::course_curriculum::{CourseCurriculumResponse, CreateCourseCurriculumRequest},
    error::app_error::AppError,
    models::course_curriculum::NewCourseCurriculum,
    repository::{
        course_curriculum_repository::CourseCurriculumRepository,
        curriculum_repository::CurriculumRepository,
        master_course_repository::MasterCourseRepository,
    },
};

pub struct CourseCurriculumService;

impl CourseCurriculumService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateCourseCurriculumRequest,
    ) -> Result<CourseCurriculumResponse, AppError> {
        MasterCourseRepository::find_by_id(conn, request.master_course_id)
            .map_err(|_| AppError::MasterCourseNotFound)?;

        CurriculumRepository::find_by_id(conn, request.curriculum_id)
            .map_err(|_| AppError::CurriculumNotFound)?;

        let query_params = HashMap::from([
            (
                "master_course_id".to_string(),
                request.master_course_id.to_string(),
            ),
            (
                "curriculum_id".to_string(),
                request.curriculum_id.to_string(),
            ),
        ]);

        if !CourseCurriculumRepository::find_all(conn, &query_params)
            .unwrap_or_else(|_| Vec::new())
            .is_empty()
        {
            return Err(AppError::CourseCurriculumAlreadyExists);
        }

        let new_course_curriculum = NewCourseCurriculum {
            master_course_id: request.master_course_id,
            curriculum_id: request.curriculum_id,
        };

        CourseCurriculumRepository::create(conn, &new_course_curriculum)
            .map_err(|_| AppError::DatabaseError)?;

        let course_curriculum = CourseCurriculumRepository::find_all(conn, &query_params)
            .map_err(|_| AppError::DatabaseError)?
            .into_iter()
            .next()
            .unwrap_or_else(|| unreachable!());

        Ok(course_curriculum.into())
    }

    pub fn get_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> Result<Vec<CourseCurriculumResponse>, AppError> {
        let course_curriculums = CourseCurriculumRepository::find_all(conn, params)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(course_curriculums.into_iter().map(|cc| cc.into()).collect())
    }

    pub fn get_by_id(
        conn: &mut PgConnection,
        course_curriculum_id: i64,
    ) -> Result<CourseCurriculumResponse, AppError> {
        let course_curriculum = CourseCurriculumRepository::find_by_id(conn, course_curriculum_id)
            .map_err(|_| AppError::CourseCurriculumNotFound)?;

        Ok(course_curriculum.into())
    }

    pub fn delete(conn: &mut PgConnection, course_curriculum_id: i64) -> Result<(), AppError> {
        CourseCurriculumRepository::find_by_id(conn, course_curriculum_id)
            .map_err(|_| AppError::CourseCurriculumNotFound)?;

        CourseCurriculumRepository::delete(conn, course_curriculum_id)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}
