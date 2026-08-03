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
        if MajorRepository::find_by_name(conn, &request.name).is_ok() {
            return Err(AppError::MajorAlreadyExists);
        }

        let new_major = NewMajor { name: request.name };

        let major =
            MajorRepository::create(conn, &new_major).map_err(|_| AppError::DatabaseError)?;

        Ok(major.into())
    }

    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<MajorResponse>, AppError> {
        let majors = MajorRepository::find_all(conn).map_err(|_| AppError::DatabaseError)?;

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
        if let Some(ref name) = request.name {
            if let Ok(existing) = MajorRepository::find_by_name(conn, name) {
                if existing.id != major_id {
                    return Err(AppError::MajorAlreadyExists);
                }
            }
        }

        let update_major = UpdateMajor { name: request.name };

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
