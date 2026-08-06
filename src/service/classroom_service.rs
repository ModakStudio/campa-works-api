use std::collections::HashMap;

use diesel::PgConnection;

use crate::{
    dto::classroom::{ClassroomResponse, CreateClassroomRequest, UpdateClassroomRequest},
    error::app_error::AppError,
    models::classroom::{NewClassroom, UpdateClassroom},
    repository::classroom_repository::ClassroomRepository,
};

pub struct ClassroomService;

impl ClassroomService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateClassroomRequest,
    ) -> Result<ClassroomResponse, AppError> {
        if ClassroomRepository::find_by_building_and_room(conn, &request.building, &request.room)
            .is_ok()
        {
            return Err(AppError::ClassroomAlreadyExists);
        }

        let new_classroom = NewClassroom {
            building: request.building,
            room: request.room,
            capacity: request.capacity,
            is_available: request.is_available.unwrap_or(true),
        };

        let classroom = ClassroomRepository::create(conn, &new_classroom)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(classroom.into())
    }

    pub fn get_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> Result<Vec<ClassroomResponse>, AppError> {
        let classrooms =
            ClassroomRepository::find_all(conn, params).map_err(|_| AppError::DatabaseError)?;

        Ok(classrooms.into_iter().map(Into::into).collect())
    }

    pub fn get_by_id(
        conn: &mut PgConnection,
        classroom_id: i64,
    ) -> Result<ClassroomResponse, AppError> {
        let classroom = ClassroomRepository::find_by_id(conn, classroom_id)
            .map_err(|_| AppError::ClassroomNotFound)?;

        Ok(classroom.into())
    }

    pub fn update(
        conn: &mut PgConnection,
        classroom_id: i64,
        request: UpdateClassroomRequest,
    ) -> Result<ClassroomResponse, AppError> {
        ClassroomRepository::find_by_id(conn, classroom_id)
            .map_err(|_| AppError::ClassroomNotFound)?;

        let update_classroom = UpdateClassroom {
            building: request.building,
            room: request.room,
            capacity: request.capacity,
            is_available: request.is_available,
        };

        let classroom = ClassroomRepository::update(conn, classroom_id, &update_classroom)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => AppError::ClassroomNotFound,
                _ => AppError::DatabaseError,
            })?;

        Ok(classroom.into())
    }

    pub fn delete(conn: &mut PgConnection, classroom_id: i64) -> Result<(), AppError> {
        ClassroomRepository::find_by_id(conn, classroom_id)
            .map_err(|_| AppError::ClassroomNotFound)?;

        ClassroomRepository::delete(conn, classroom_id).map_err(|e| match e {
            diesel::result::Error::NotFound => AppError::ClassroomNotFound,
            _ => AppError::DatabaseError,
        })?;

        Ok(())
    }
}
