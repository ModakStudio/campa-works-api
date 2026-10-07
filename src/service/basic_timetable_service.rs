use diesel::prelude::*;

use crate::{
    dto::basic_timetable::{
        BasicTimetableResponse, BasicTimetableSlotRequest, CreateBasicTimetableRequest,
        UpdateBasicTimetableRequest,
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
        let total_min = calculate_total_min(&request.slots)?;
        let model = BasicTimetableRepository::create(
            conn,
            &NewBasicTimetableModel { total_min },
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

        let slots = request
            .slots
            .map(|slots| {
                let total_min = calculate_total_min(&slots);
                total_min.map(|total_min| (slots, total_min))
            })
            .transpose()?;
        let total_min = slots.as_ref().map(|(_, total_min)| *total_min);
        let slots = slots.map(|(slots, _)| {
            slots
                .iter()
                .map(NewBasicTimetableSlot::from)
                .collect::<Vec<_>>()
        });
        let model = BasicTimetableRepository::update(
            conn,
            model_id,
            &UpdateBasicTimetableModel { total_min },
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

fn calculate_total_min(slots: &[BasicTimetableSlotRequest]) -> Result<i32, AppError> {
    let mut durations = slots.iter().map(|slot| slot.end_time - slot.start_time);
    let total_min: i64 = durations
        .clone()
        .map(|duration| duration.num_minutes())
        .sum();

    if slots.is_empty()
        || durations.any(|duration| duration.num_minutes() < 0)
        || total_min <= 0
        || total_min > i32::MAX as i64
    {
        return Err(AppError::InvalidBasicTimetableSlots);
    }

    Ok(total_min as i32)
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
