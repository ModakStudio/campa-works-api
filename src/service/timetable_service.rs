use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    dto::timetable::{CreateTimetableRequest, TimetableResponse, UpdateTimetableRequest},
    error::app_error::AppError,
    models::timetable::{NewTimetable, UpdateTimetable},
    repository::{
        classroom_repository::ClassroomRepository,
        course_assignment_repository::CourseAssignmentRepository,
        timetable_repository::TimetableRepository,
    },
    service::course_service::CourseService,
};

pub struct TimetableService;

impl TimetableService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateTimetableRequest,
    ) -> Result<TimetableResponse, AppError> {
        CourseAssignmentRepository::find_by_id(conn, request.assignment_id)
            .map_err(|_| AppError::CourseAssignmentNotFound)?;

        ClassroomRepository::find_by_id(conn, request.classroom_id)
            .map_err(|_| AppError::ClassroomNotFound)?;

        if TimetableRepository::find_overlapping_timetables(
            conn,
            request.classroom_id,
            request.day_of_week,
            request.start_time,
            request.end_time,
        )
        .map_err(|_| AppError::DatabaseError)?
        .len()
            > 0
        {
            return Err(AppError::TimetableOverlap);
        }

        let new_timetable = NewTimetable {
            assignment_id: request.assignment_id,
            classroom_id: request.classroom_id,
            day_of_week: request.day_of_week,
            start_time: request.start_time,
            end_time: request.end_time,
        };

        let timetable_id = TimetableRepository::create(conn, &new_timetable)
            .map_err(|_| AppError::DatabaseError)?
            .id;
        let timetable = TimetableRepository::find_by_id(conn, timetable_id)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(timetable.into())
    }

    pub fn create_all_in_new_semester(
        conn: &mut PgConnection,
        new_semester_id: i64,
    ) -> Result<(), AppError> {
        // Create all courses in the new semester
        CourseService::create_all_in_new_semester(conn, new_semester_id)?;

        // ToDo: Assign professors to courses in the new semester

        // ToDo: Create timetables for the new semester based on the courses and professors

        Ok(())
    }

    pub fn get_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> Result<Vec<TimetableResponse>, AppError> {
        let timetables =
            TimetableRepository::find_all(conn, params).map_err(|_| AppError::DatabaseError)?;

        Ok(timetables.into_iter().map(Into::into).collect())
    }

    pub fn get_by_id(conn: &mut PgConnection, id: i64) -> Result<TimetableResponse, AppError> {
        let timetable =
            TimetableRepository::find_by_id(conn, id).map_err(|_| AppError::TimetableNotFound)?;

        Ok(timetable.into())
    }

    pub fn get_overlapping_timetables(
        conn: &mut PgConnection,
        request: CreateTimetableRequest,
    ) -> Result<Vec<TimetableResponse>, AppError> {
        let overlapping_timetables = TimetableRepository::find_overlapping_timetables(
            conn,
            request.classroom_id,
            request.day_of_week,
            request.start_time,
            request.end_time,
        )
        .map_err(|_| AppError::DatabaseError)?;

        Ok(overlapping_timetables.into_iter().map(Into::into).collect())
    }

    pub fn update(
        conn: &mut PgConnection,
        timetable_id: i64,
        request: UpdateTimetableRequest,
    ) -> Result<TimetableResponse, AppError> {
        TimetableRepository::find_by_id(conn, timetable_id)
            .map_err(|_| AppError::TimetableNotFound)?;

        if let Some(assignment_id) = request.assignment_id {
            CourseAssignmentRepository::find_by_id(conn, assignment_id)
                .map_err(|_| AppError::CourseAssignmentNotFound)?;
        }

        if let Some(classroom_id) = request.classroom_id {
            ClassroomRepository::find_by_id(conn, classroom_id)
                .map_err(|_| AppError::ClassroomNotFound)?;
        }

        let updated_timetable = UpdateTimetable {
            assignment_id: request.assignment_id,
            classroom_id: request.classroom_id,

            day_of_week: request.day_of_week,

            start_time: request.start_time,
            end_time: request.end_time,
        };

        TimetableRepository::update(conn, timetable_id, &updated_timetable)
            .map_err(|_| AppError::DatabaseError)?;

        let updated_timetable = TimetableRepository::find_by_id(conn, timetable_id)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(updated_timetable.into())
    }

    pub fn delete(conn: &mut PgConnection, timetable_id: i64) -> Result<(), AppError> {
        TimetableRepository::find_by_id(conn, timetable_id)
            .map_err(|_| AppError::TimetableNotFound)?;

        TimetableRepository::delete(conn, timetable_id).map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}
