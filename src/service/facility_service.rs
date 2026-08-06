use std::collections::HashMap;

use diesel::PgConnection;

use crate::{
    dto::facility::{CreateFacilityRequest, FacilityResponse, UpdateFacilityRequest},
    error::app_error::AppError,
    models::facility::{NewFacility, UpdateFacility},
    repository::facility_repository::FacilityRepository,
};

pub struct FacilityService;

impl FacilityService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateFacilityRequest,
    ) -> Result<FacilityResponse, AppError> {
        if FacilityRepository::find_by_name(conn, &request.name).is_ok() {
            return Err(AppError::FacilityAlreadyExists);
        }

        let new_facility = NewFacility {
            name: request.name,
            description: request.description,
        };

        let facility =
            FacilityRepository::create(conn, &new_facility).map_err(|_| AppError::DatabaseError)?;

        Ok(facility.into())
    }

    pub fn get_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> Result<Vec<FacilityResponse>, AppError> {
        let facilities =
            FacilityRepository::find_all(conn, params).map_err(|_| AppError::DatabaseError)?;

        Ok(facilities.into_iter().map(Into::into).collect())
    }

    pub fn get_by_id(
        conn: &mut PgConnection,
        facility_id: i64,
    ) -> Result<FacilityResponse, AppError> {
        let facility = FacilityRepository::find_by_id(conn, facility_id)
            .map_err(|_| AppError::FacilityNotFound)?;

        Ok(facility.into())
    }

    pub fn update(
        conn: &mut PgConnection,
        facility_id: i64,
        request: UpdateFacilityRequest,
    ) -> Result<FacilityResponse, AppError> {
        if let Some(ref name) = request.name {
            if let Ok(existing) = FacilityRepository::find_by_name(conn, name) {
                if existing.id != facility_id {
                    return Err(AppError::FacilityAlreadyExists);
                }
            }
        }

        let update_facility = UpdateFacility {
            name: request.name,
            description: request.description,
        };

        let facility = FacilityRepository::update(conn, facility_id, &update_facility).map_err(
            |e| match e {
                diesel::result::Error::NotFound => AppError::FacilityNotFound,
                _ => AppError::DatabaseError,
            },
        )?;

        Ok(facility.into())
    }

    pub fn delete(conn: &mut PgConnection, facility_id: i64) -> Result<(), AppError> {
        FacilityRepository::find_by_id(conn, facility_id)
            .map_err(|_| AppError::FacilityNotFound)?;

        FacilityRepository::delete(conn, facility_id).map_err(|e| match e {
            diesel::result::Error::NotFound => AppError::FacilityNotFound,
            _ => AppError::DatabaseError,
        })?;

        Ok(())
    }
}
