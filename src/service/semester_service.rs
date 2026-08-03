use diesel::PgConnection;

use crate::{
    dto::semester::{CreateSemesterRequest, SemesterResponse, UpdateSemesterRequest},
    error::app_error::AppError,
    models::semester::{NewSemester, UpdateSemester},
    repository::semester_repository::SemesterRepository,
};

pub struct SemesterService;

impl SemesterService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateSemesterRequest,
    ) -> Result<SemesterResponse, AppError> {
        if SemesterRepository::find_by_year_and_semester(conn, request.year, request.semester_)
            .is_ok()
        {
            return Err(AppError::SemesterAlreadyExists);
        }

        let new_semester = NewSemester {
            year: request.year,
            semester_: request.semester_,
            status: crate::models::enums::SemesterStatus::Open,
        };

        let semester =
            SemesterRepository::create(conn, &new_semester).map_err(|_| AppError::DatabaseError)?;

        Ok(semester.into())
    }

    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<SemesterResponse>, AppError> {
        let semesters = SemesterRepository::find_all(conn).map_err(|_| AppError::DatabaseError)?;

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
            status: request.status,
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
