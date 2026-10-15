use std::collections::HashMap;

use diesel::PgConnection;

use crate::{
    dto::curriculum::{CreateCurriculumRequest, CurriculumResponse},
    error::app_error::AppError,
    models::curriculum::NewCurriculum,
    repository::{
        curriculum_repository::CurriculumRepository, major_repository::MajorRepository,
        semester_repository::SemesterRepository,
    },
};

pub struct CurriculumService;

impl CurriculumService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateCurriculumRequest,
    ) -> Result<CurriculumResponse, AppError> {
        SemesterRepository::find_by_id(conn, request.semester_id)
            .map_err(|_| AppError::SemesterNotFound)?;

        MajorRepository::find_by_id(conn, request.major_id).map_err(|_| AppError::MajorNotFound)?;

        let query_params = HashMap::from([
            ("semester_id".to_string(), request.semester_id.to_string()),
            ("major_id".to_string(), request.major_id.to_string()),
            ("grade".to_string(), request.grade.to_string()),
        ]);

        if !CurriculumRepository::find_all(conn, &query_params)
            .unwrap_or_else(|_| Vec::new())
            .is_empty()
        {
            return Err(AppError::CurriculumAlreadyExists);
        }

        let new_curriculum = NewCurriculum {
            semester_id: request.semester_id,
            major_id: request.major_id,
            grade: request.grade,
        };

        CurriculumRepository::create(conn, &new_curriculum).map_err(|_| AppError::DatabaseError)?;

        let curriculum = CurriculumRepository::find_all(conn, &query_params)
            .map_err(|_| AppError::DatabaseError)?
            .into_iter()
            .next()
            .unwrap_or_else(|| unreachable!());

        Ok(curriculum.into())
    }

    pub fn get_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> Result<Vec<CurriculumResponse>, AppError> {
        let curriculums =
            CurriculumRepository::find_all(conn, params).map_err(|_| AppError::DatabaseError)?;

        Ok(curriculums.into_iter().map(Into::into).collect())
    }

    pub fn get_by_id(
        conn: &mut PgConnection,
        curriculum_id: i64,
    ) -> Result<CurriculumResponse, AppError> {
        let curriculum = CurriculumRepository::find_by_id(conn, curriculum_id)
            .map_err(|_| AppError::CurriculumNotFound)?;

        Ok(curriculum.into())
    }

    pub fn get_curriculums_for_all_grades_from_semester_id(
        conn: &mut PgConnection,
        semester_id: i64,
    ) -> Result<Vec<CurriculumResponse>, AppError> {
        let mut curriculums = Vec::new();

        for major in
            MajorRepository::find_all(conn, &HashMap::new()).map_err(|_| AppError::DatabaseError)?
        {
            for grade in 1..=4 {
                let last_semester =
                    SemesterRepository::find_previous_year_semester(conn, semester_id, grade - 1)
                        .map_err(|_| AppError::SemesterNotFound)?;

                if let Some(curriculum) =
                    CurriculumRepository::find_by_semester_id_and_major_id_and_grade(
                        conn,
                        last_semester.id,
                        major.id,
                        grade,
                    )
                    .map_err(|_| AppError::CurriculumNotFound)
                    .ok()
                {
                    curriculums.push(curriculum.into());
                }
            }
        }

        Ok(curriculums)
    }

    pub fn delete(conn: &mut PgConnection, curriculum_id: i64) -> Result<(), AppError> {
        CurriculumRepository::find_by_id(conn, curriculum_id)
            .map_err(|_| AppError::CurriculumNotFound)?;

        CurriculumRepository::delete(conn, curriculum_id).map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}
