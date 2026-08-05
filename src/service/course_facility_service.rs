use diesel::PgConnection;

use crate::{
    dto::course_facility::{CourseFacilityResponse, CreateCourseFacilityRequest},
    error::app_error::AppError,
    models::course_facility::NewCourseFacility,
    repository::{
        course_facility_repository::CourseFacilityRepository,
        facility_repository::FacilityRepository, master_course_repository::MasterCourseRepository,
    },
};

pub struct CourseFacilityService;

impl CourseFacilityService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateCourseFacilityRequest,
    ) -> Result<CourseFacilityResponse, AppError> {
        MasterCourseRepository::find_by_id(conn, request.master_course_id)
            .map_err(|_| AppError::MasterCourseNotFound)?;

        FacilityRepository::find_by_id(conn, request.facility_id)
            .map_err(|_| AppError::FacilityNotFound)?;

        if CourseFacilityRepository::find_by_master_course_id_and_facility_id(
            conn,
            request.master_course_id,
            request.facility_id,
        )
        .is_ok()
        {
            return Err(AppError::CourseFacilityAlreadyExists);
        }

        let new_course_facility = NewCourseFacility {
            master_course_id: request.master_course_id,
            facility_id: request.facility_id,
        };

        CourseFacilityRepository::create(conn, &new_course_facility)
            .map_err(|_| AppError::DatabaseError)?;

        let course_facility = CourseFacilityRepository::find_by_master_course_id_and_facility_id(
            conn,
            request.master_course_id,
            request.facility_id,
        )
        .map_err(|_| AppError::DatabaseError)?;

        Ok(course_facility.into())
    }

    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<CourseFacilityResponse>, AppError> {
        let course_facilities =
            CourseFacilityRepository::find_all(conn).map_err(|_| AppError::DatabaseError)?;

        Ok(course_facilities.into_iter().map(Into::into).collect())
    }

    pub fn get_by_id(conn: &mut PgConnection, id: i64) -> Result<CourseFacilityResponse, AppError> {
        let course_facility = CourseFacilityRepository::find_by_id(conn, id)
            .map_err(|_| AppError::CourseFacilityNotFound)?;

        Ok(course_facility.into())
    }

    pub fn delete(conn: &mut PgConnection, id: i64) -> Result<(), AppError> {
        CourseFacilityRepository::find_by_id(conn, id)
            .map_err(|_| AppError::CourseFacilityNotFound)?;

        CourseFacilityRepository::delete(conn, id).map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}
