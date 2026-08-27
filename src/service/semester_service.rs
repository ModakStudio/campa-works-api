use std::collections::HashMap;

use diesel::PgConnection;

use crate::{
    dto::semester::{CreateSemesterRequest, SemesterResponse, UpdateSemesterRequest},
    error::app_error::AppError,
    models::{
        enums::SemesterStatus,
        semester::{NewSemester, UpdateSemester},
    },
    repository::semester_repository::SemesterRepository,
};

pub struct SemesterService;

impl SemesterService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateSemesterRequest,
    ) -> Result<SemesterResponse, AppError> {
        let query_params = HashMap::from([
            ("year".to_string(), request.year.to_string()),
            ("semester_".to_string(), request.semester_.to_string()),
        ]);

        if !SemesterRepository::find_all(conn, &query_params)
            .unwrap_or_else(|_| Vec::new())
            .is_empty()
        {
            return Err(AppError::SemesterAlreadyExists);
        }

        let new_semester = NewSemester {
            year: request.year,
            semester_: request.semester_,
            semester_status: SemesterStatus::Open,
        };

        let semester =
            SemesterRepository::create(conn, &new_semester).map_err(|_| AppError::DatabaseError)?;

        Ok(semester.into())
    }

    pub fn get_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> Result<Vec<SemesterResponse>, AppError> {
        let semesters =
            SemesterRepository::find_all(conn, params).map_err(|_| AppError::DatabaseError)?;

        Ok(semesters.into_iter().map(Into::into).collect())
    }

    pub fn get_by_id(
        conn: &mut PgConnection,
        semester_id: i64,
    ) -> Result<SemesterResponse, AppError> {
        let semester = SemesterRepository::find_by_id(conn, semester_id)
            .map_err(|_| AppError::SemesterNotFound)?;

        Ok(semester.into())
    }

    pub fn update(
        conn: &mut PgConnection,
        semester_id: i64,
        request: UpdateSemesterRequest,
    ) -> Result<SemesterResponse, AppError> {
        let update = UpdateSemester {
            year: request.year,
            semester_: request.semester_,
            semester_status: request.semester_status,
        };

        let semester =
            SemesterRepository::update(conn, semester_id, &update).map_err(|e| match e {
                diesel::result::Error::NotFound => AppError::SemesterNotFound,
                _ => AppError::DatabaseError,
            })?;

        Ok(semester.into())
    }

    pub fn delete(conn: &mut PgConnection, semester_id: i64) -> Result<(), AppError> {
        SemesterRepository::find_by_id(conn, semester_id)
            .map_err(|_| AppError::SemesterNotFound)?;

        SemesterRepository::delete(conn, semester_id).map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}
