use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use crate::models::{
    basic_timetable::{BasicTimetableModel, BasicTimetableSlot},
    enums::DayOfWeek,
};

#[derive(Debug, Deserialize)]
pub struct BasicTimetableSlotRequest {
    pub day_of_week: DayOfWeek,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateBasicTimetableRequest {
    pub total_min: i32,
    pub slots: Vec<BasicTimetableSlotRequest>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBasicTimetableRequest {
    pub total_min: Option<i32>,
    pub slots: Option<Vec<BasicTimetableSlotRequest>>,
}

#[derive(Debug, Serialize)]
pub struct BasicTimetableSlotResponse {
    pub id: i64,
    pub day_of_week: DayOfWeek,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

#[derive(Debug, Serialize)]
pub struct BasicTimetableResponse {
    pub id: i64,
    pub total_min: i32,
    pub slots: Vec<BasicTimetableSlotResponse>,
}

impl From<(BasicTimetableModel, Vec<BasicTimetableSlot>)> for BasicTimetableResponse {
    fn from((model, slots): (BasicTimetableModel, Vec<BasicTimetableSlot>)) -> Self {
        Self {
            id: model.id,
            total_min: model.total_min,
            slots: slots
                .into_iter()
                .map(|slot| BasicTimetableSlotResponse {
                    id: slot.id,
                    day_of_week: slot.day_of_week,
                    start_time: slot.start_time,
                    end_time: slot.end_time,
                })
                .collect(),
        }
    }
}
