use diesel::PgConnection;

use crate::{
    dto::classroom_facility::{ClassroomFacilityResponse, CreateClassroomFacilityRequest},
    error::app_error::AppError,
    models::classroom_facility::NewClassroomFacility,
    repository::{
        classroom_facility_repository::ClassroomFacilityRepository,
        classroom_repository::ClassroomRepository, facility_repository::FacilityRepository,
    },
};

pub struct ClassroomFacilityService;

impl ClassroomFacilityService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateClassroomFacilityRequest,
    ) -> Result<ClassroomFacilityResponse, AppError> {
        ClassroomRepository::find_by_id(conn, request.classroom_id)
            .map_err(|_| AppError::ClassroomNotFound)?;

        FacilityRepository::find_by_id(conn, request.facility_id)
            .map_err(|_| AppError::FacilityNotFound)?;

        if ClassroomFacilityRepository::find_by_classroom_id_and_facility_id(
            conn,
            request.classroom_id,
            request.facility_id,
        )
        .is_ok()
        {
            return Err(AppError::ClassroomFacilityAlreadyExists);
        }

        let new_classroom_facility = NewClassroomFacility {
            classroom_id: request.classroom_id,
            facility_id: request.facility_id,
        };

        ClassroomFacilityRepository::create(conn, &new_classroom_facility)
            .map_err(|_| AppError::DatabaseError)?;

        let classroom_facility = ClassroomFacilityRepository::find_by_classroom_id_and_facility_id(
            conn,
            request.classroom_id,
            request.facility_id,
        )
        .map_err(|_| AppError::DatabaseError)?;

        Ok(classroom_facility.into())
    }

    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<ClassroomFacilityResponse>, AppError> {
        let classroom_facilities =
            ClassroomFacilityRepository::find_all(conn).map_err(|_| AppError::DatabaseError)?;

        Ok(classroom_facilities.into_iter().map(Into::into).collect())
    }

    pub fn get_by_id(
        conn: &mut PgConnection,
        id: i64,
    ) -> Result<ClassroomFacilityResponse, AppError> {
        let classroom_facility = ClassroomFacilityRepository::find_by_id(conn, id)
            .map_err(|_| AppError::ClassroomFacilityNotFound)?;

        Ok(classroom_facility.into())
    }

    pub fn delete(conn: &mut PgConnection, id: i64) -> Result<(), AppError> {
        ClassroomFacilityRepository::find_by_id(conn, id)
            .map_err(|_| AppError::ClassroomFacilityNotFound)?;

        ClassroomFacilityRepository::delete(conn, id).map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}
