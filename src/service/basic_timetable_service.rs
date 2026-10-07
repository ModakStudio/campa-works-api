use diesel::prelude::*;

use crate::{
    dto::basic_timetable::{
        BasicTimetableResponse, CreateBasicTimetableRequest, UpdateBasicTimetableRequest,
    },
    error::app_error::AppError,
    models::basic_timetable::{
        NewBasicTimetableModel, NewBasicTimetableSlot, UpdateBasicTimetableModel,
    },
    repository::basic_timetable_repository::BasicTimetableRepository,
};

pub struct BasicTimetableService;

impl BasicTimetableService {
    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<BasicTimetableResponse>, AppError> {
        BasicTimetableRepository::find_all(conn)
            .map(|models| models.into_iter().map(Into::into).collect())
            .map_err(|_| AppError::DatabaseError)
    }

    pub fn get_by_id(
        conn: &mut PgConnection,
        model_id: i64,
    ) -> Result<BasicTimetableResponse, AppError> {
        BasicTimetableRepository::find_by_id(conn, model_id)
            .map(Into::into)
            .map_err(|_| AppError::BasicTimetableModelNotFound)
    }

    pub fn create(
        conn: &mut PgConnection,
        request: CreateBasicTimetableRequest,
    ) -> Result<BasicTimetableResponse, AppError> {
        let model = BasicTimetableRepository::create(
            conn,
            &NewBasicTimetableModel {
                total_min: request.total_min,
            },
            &request
                .slots
                .iter()
                .map(NewBasicTimetableSlot::from)
                .collect::<Vec<_>>(),
        )
        .map_err(|_| AppError::DatabaseError)?;

        BasicTimetableRepository::find_by_id(conn, model.id)
            .map(Into::into)
            .map_err(|_| AppError::DatabaseError)
    }

    pub fn update(
        conn: &mut PgConnection,
        model_id: i64,
        request: UpdateBasicTimetableRequest,
    ) -> Result<BasicTimetableResponse, AppError> {
        BasicTimetableRepository::find_by_id(conn, model_id)
            .map_err(|_| AppError::BasicTimetableModelNotFound)?;

        let slots = request.slots.map(|slots| {
            slots
                .iter()
                .map(NewBasicTimetableSlot::from)
                .collect::<Vec<_>>()
        });
        let model = BasicTimetableRepository::update(
            conn,
            model_id,
            &UpdateBasicTimetableModel {
                total_min: request.total_min,
            },
            slots.as_deref(),
        )
        .map_err(|_| AppError::DatabaseError)?;

        Ok(model.into())
    }

    pub fn delete(conn: &mut PgConnection, model_id: i64) -> Result<(), AppError> {
        BasicTimetableRepository::find_by_id(conn, model_id)
            .map_err(|_| AppError::BasicTimetableModelNotFound)?;
        BasicTimetableRepository::delete(conn, model_id).map_err(|_| AppError::DatabaseError)?;
        Ok(())
    }
}

impl From<&crate::dto::basic_timetable::BasicTimetableSlotRequest> for NewBasicTimetableSlot {
    fn from(slot: &crate::dto::basic_timetable::BasicTimetableSlotRequest) -> Self {
        Self {
            day_of_week: slot.day_of_week,
            start_time: slot.start_time,
            end_time: slot.end_time,
        }
    }
}
