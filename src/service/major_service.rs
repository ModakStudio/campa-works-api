use std::collections::HashMap;

use diesel::PgConnection;

use crate::{
    dto::major::{CreateMajorRequest, MajorResponse, UpdateMajorRequest},
    error::app_error::AppError,
    models::major::{NewMajor, UpdateMajor},
    repository::major_repository::MajorRepository,
};

pub struct MajorService;

impl MajorService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateMajorRequest,
    ) -> Result<MajorResponse, AppError> {
        let query_params = HashMap::from([
            ("major_name".into(), request.major_name.clone()),
            ("major_code".into(), request.major_code.clone()),
        ]);
        if !MajorRepository::find_all(conn, &query_params)
            .unwrap_or_else(|_| Vec::new())
            .is_empty()
        {
            return Err(AppError::MajorAlreadyExists);
        }

        let new_major = NewMajor {
            major_name: request.major_name,
            major_code: request.major_code,

            major_status: request.major_status,
        };

        let major =
            MajorRepository::create(conn, &new_major).map_err(|_| AppError::DatabaseError)?;

        Ok(major.into())
    }

    pub fn get_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> Result<Vec<MajorResponse>, AppError> {
        let majors =
            MajorRepository::find_all(conn, params).map_err(|_| AppError::DatabaseError)?;

        Ok(majors.into_iter().map(Into::into).collect())
    }

    pub fn get_by_id(conn: &mut PgConnection, major_id: i64) -> Result<MajorResponse, AppError> {
        let major =
            MajorRepository::find_by_id(conn, major_id).map_err(|_| AppError::MajorNotFound)?;

        Ok(major.into())
    }

    pub fn update(
        conn: &mut PgConnection,
        major_id: i64,
        request: UpdateMajorRequest,
    ) -> Result<MajorResponse, AppError> {
        let update_major = UpdateMajor {
            major_name: request.major_name,
            major_code: request.major_code,
            major_status: request.major_status,
        };

        let major =
            MajorRepository::update(conn, major_id, &update_major).map_err(|e| match e {
                diesel::result::Error::NotFound => AppError::MajorNotFound,
                _ => AppError::DatabaseError,
            })?;

        Ok(major.into())
    }

    pub fn delete(conn: &mut PgConnection, major_id: i64) -> Result<(), AppError> {
        MajorRepository::find_by_id(conn, major_id).map_err(|_| AppError::MajorNotFound)?;

        MajorRepository::delete(conn, major_id).map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}
